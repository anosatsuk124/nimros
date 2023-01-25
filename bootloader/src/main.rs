#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

use core::fmt::Write;

use core::fmt;
use log::info;
use uefi::{
    prelude::*,
    proto::media::file::{self, File, FileAttribute, FileMode},
    table::boot::{self, MemoryDescriptor, MemoryMapKey},
    Result,
};

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello World");
    let memmap_buf = [0x00; 4096 * 4];
    let map = MemoryMap::get_memory_map(system_table.boot_services(), memmap_buf)
        .expect("Couldn't get the memory map");
    info!("Memory map: {:?}", &map);
    if let Ok(mut fs) = system_table
        .boot_services()
        .get_image_file_system(image_handle)
    {
        let mut root_dir = fs.open_volume().expect("Cannot open the root directory");

        let mut file = root_dir
            .open(
                cstr16!("memmap"),
                FileMode::CreateReadWrite,
                FileAttribute::empty(),
            )
            .expect("Cannot open the file")
            .into_regular_file()
            .expect("This is not a regular file.");

        map.save(&mut file).expect("Couldn't save");
        file.close();
    };

    loop {}
    return Status::SUCCESS;
}

struct RegularFileWriter<'a>(&'a mut file::RegularFile);

impl<'a> RegularFileWriter<'a> {
    fn new(regular_file: &'a mut file::RegularFile) -> Self {
        RegularFileWriter(regular_file)
    }
}

impl<'a> fmt::Write for RegularFileWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.0.write(s.as_bytes()).is_err() {
            panic!("Couldn't write");
        }
        fmt::Result::Ok(())
    }
}

#[derive(Debug)]
struct MemoryMap<const N: usize> {
    map_buffer: Option<[u8; N]>,
    map_key: MemoryMapKey,
    descriptor_size: usize,
    descriptor_version: u32,
}

impl<const N: usize> MemoryMap<N> {
    fn get_memory_map(
        boot_services: &BootServices,
        mut map_buffer: [u8; N],
    ) -> Result<MemoryMap<N>> {
        let mut map;
        match boot_services.memory_map(&mut map_buffer) {
            Ok((map_key, map_iter)) => {
                map = MemoryMap {
                    map_buffer: None,
                    map_key,
                    descriptor_size: map_iter.len(),
                    descriptor_version: boot::MEMORY_DESCRIPTOR_VERSION,
                };
            }
            Err(e) => return Err(e),
        }
        map.map_buffer = Some(map_buffer);
        Ok(map)
    }

    fn save(&self, file: &mut file::RegularFile) -> Result<()> {
        let mut writer = RegularFileWriter::new(file);
        writeln!(
            &mut writer,
            "Index, Type, PhysicalStart, NumOfPage, Attribute"
        )
        .expect("Couldn't write.");

        if let Some(map_buffer) = self.map_buffer {
            for (i, phys_addr) in map_buffer.chunks(self.descriptor_size).enumerate() {
                if let Some(desc) =
                    unsafe { (*phys_addr.as_ptr() as *const MemoryDescriptor).as_ref() }
                {
                    writeln!(
                        &mut writer,
                        "{:?}, {:?}, {:?}, {:?}, {:?}",
                        i, desc.ty, desc.phys_start, desc.page_count, desc.att
                    )
                    .expect("Couldn't write.")
                };
            }
        }
        Ok(())
    }
}
