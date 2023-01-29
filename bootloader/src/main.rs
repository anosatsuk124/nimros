#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]
#![feature(ptr_metadata)]

use core::{fmt::Write, mem::size_of};
use mikanlib::graphics::{FrameBufferConfig, self};
use uefi::proto::console::gop::{GraphicsOutput, PixelFormat};

use core::{fmt, mem};
use log::info;
use uefi::data_types::{Align, PhysicalAddress};
use uefi::proto::media::file::FileInfo;
use uefi::table::boot::{MemoryType, OpenProtocolParams};
use uefi::{
    prelude::*,
    proto::media::file::{self, File, FileAttribute, FileMode},
    table::boot::{self, MemoryDescriptor, MemoryMapKey},
    Result,
};

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let frame_buffer_config = {
        let boot_services = system_table.boot_services();
        let gop_handle = boot_services
            .get_handle_for_protocol::<GraphicsOutput>()
            .expect("Couldn't get the GOP handle");
        let gop_params = OpenProtocolParams {
            handle: gop_handle,
            agent: image_handle,
            controller: None,
        };
        let mut gop = unsafe {
            boot_services.open_protocol::<GraphicsOutput>(
                gop_params,
                boot::OpenProtocolAttributes::GetProtocol,
            )
        }
        .expect("Couldn't open the GOP");
        let gop_mode_info = &gop.current_mode_info();
        info!(
            "Resolution: {:?}, Pixel Format: {:?}, {:?} pixels/line",
            &gop_mode_info.resolution(),
            &gop_mode_info.pixel_format(),
            &gop_mode_info.stride()
        );
        let mut frame_buffer = gop.frame_buffer();
        let pixel_format = match gop_mode_info.pixel_format() {
            PixelFormat::Rgb => graphics::PixelFormat::PixelRGBResv8bitPerColor,
            PixelFormat::Bgr => graphics::PixelFormat::PixelBGRResv8bitPerColor,
            _ => unimplemented!(),
        };
        FrameBufferConfig {
            frame_buffer: frame_buffer.as_mut_ptr(),
            pixels_per_scan_line: gop_mode_info.stride(),
            h_resolution: gop_mode_info.resolution().0,
            v_resolution: gop_mode_info.resolution().1,
            pixel_format,
            size: frame_buffer.size(),
        }
    };

    let mut memmap_buf = [0x00; 4096 * 4];
    let map = MemoryMap::get_memory_map(system_table.boot_services(), memmap_buf)
        .expect("Couldn't get the memory map");
    const KERNEL_BASE_ADDR: PhysicalAddress = 0x100000;
    {
        let mut fs = system_table
            .boot_services()
            .get_image_file_system(image_handle)
            .expect("Couldn't get the file system");
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

        let mut kernel_file = root_dir
            .open(cstr16!("\\kernel"), FileMode::Read, FileAttribute::empty())
            .expect("Cannot open the kernel");
        // FIXME: This size is a hacky value.
        const KERNEL_INFO_SIZE: usize = size_of::<&FileInfo>() * 8;
        let mut kernel_info_buffer = [0x00; KERNEL_INFO_SIZE];
        let kernel_info_buffer: &mut [u8] =
            FileInfo::align_buf(&mut kernel_info_buffer).expect("Cannot align");
        let kernel_info: &FileInfo = kernel_file
            .get_info(kernel_info_buffer)
            .expect("Couldn't get the kernel file info.");

        let base_addr = system_table
            .boot_services()
            .allocate_pages(
                boot::AllocateType::Address(KERNEL_BASE_ADDR),
                MemoryType::LOADER_DATA,
                ((kernel_info.file_size() + 0xfff) / 0x1000) as usize,
            )
            .expect("Couldn't allocate pages");

        let allocated_buffer = unsafe {
            core::slice::from_raw_parts_mut(
                (base_addr) as *mut u8,
                kernel_info.file_size() as usize,
            )
        };

        let mut kernel_file = kernel_file
            .into_regular_file()
            .expect("Cannot convert into a regular file");

        kernel_file
            .read(allocated_buffer)
            .expect("Couldn't read the kernel");

        kernel_file.close();

        info!("file size: {:x}", kernel_info.file_size());
        info!("buffer ptr: {:?}", allocated_buffer.as_ptr());
        let _memmap = MemoryMap::get_memory_map(system_table.boot_services(), memmap_buf);
    }
    type EntryPointType = extern "sysv64" fn(&FrameBufferConfig) -> !;

    // HACK: This 0x1000 is a hacky value. It should be refactored!
    const ENTRY_POINT_OFFSET: u64 = 0x1000;
    let kernel_main_addr = unsafe {
        core::slice::from_raw_parts(
            (KERNEL_BASE_ADDR + ENTRY_POINT_OFFSET) as *mut u8,
            mem::size_of::<EntryPointType>(),
        )
        .as_ptr()
    } as u64;
    info!("base addr: {:x}", KERNEL_BASE_ADDR);
    info!("main addr: {:x}", kernel_main_addr);
    let entry_point: EntryPointType =
        unsafe { mem::transmute::<u64, EntryPointType>(kernel_main_addr) };
    if system_table.exit_boot_services(image_handle, &mut memmap_buf).is_ok() {
        entry_point(&frame_buffer_config);
    };

    Status::SUCCESS
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

#[allow(dead_code)]
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
