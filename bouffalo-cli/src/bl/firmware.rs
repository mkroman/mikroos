use std::convert::TryInto;
use std::io::{self, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;

/// The default entry point when the user doesn't provide one when using the `FirmwareBuilder`
const DEFAULT_ENTRY_POINT: u32 = 0x2100_0000;

/// The size of the flash config structure, excluding the magic header and the crc32
const FLASH_CONFIG_STRUCT_SIZE: usize = 84;

/// The size of the clock config structure, excluding the magic header and the crc32
const CLOCK_CONFIG_STRUCT_SIZE: usize = 8;

/// The size of the boot header structure, excluding the magic header and the crc32
const BOOT_HEADER_STRUCT_SIZE: usize = 164;

/// Clock config validation errors
#[derive(Error, Debug)]
pub enum ClockConfigError {
    #[error("The magic header value is invalid: {:?}", _0)]
    InvalidMagicHeader([u8; 4]),
}

/// Boot header validation errors
#[derive(Error, Debug)]
pub enum BootHeaderError {
    #[error("The magic header value is invalid: {:?}", _0)]
    InvalidMagicHeader([u8; 4]),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Boot header error: {}", _0)]
    BootHeaderError(#[from] BootHeaderError),

    #[error("Clock config error: {}", _0)]
    ClockConfigError(#[from] ClockConfigError),

    #[error("I/O error: {}", _0)]
    IoError(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum BuilderError {
    #[error("Missing flash_config value in FirmwareBuilder")]
    MissingFlashConfig,
}

/// Indicates which CPU the firmware is for
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Cpu {
    Cpu0,
    Cpu1,
}

impl Cpu {
    /// Converts the CPU to a magic header value as little endian bytes
    pub fn to_magic_bytes(self) -> [u8; 4] {
        match self {
            Cpu::Cpu0 => *b"BFNP",
            Cpu::Cpu1 => *b"BFAP",
        }
    }
}

impl Default for Cpu {
    fn default() -> Cpu {
        Cpu::Cpu0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Firmware {
    cpu: Cpu,
    /// The boot header revision?
    revision: u32,
    /// The flash configuration magic header
    flash_config: FlashConfig,

    /// The clock configuration parameters
    clock_config: ClockConfig,

    /// Boot configuration flags
    boot_config: u32,

    /// Image segment info
    image_segment_info: u32,

    /// The entry point of the written firmware image
    entry_point: u32,

    /// Image RAM addr or flash offset
    image_start: u32,

    /// SHA-256 hash of the whole image
    hash: [u8; 32],

    /// The CRC32 checksum for the boot header
    crc32: u32,
}

#[derive(Debug, Copy, Default, Clone, Eq, PartialEq)]
pub struct ClockConfig {
    /// PLL crystal type
    // TODO: Create enum type
    // https://github.com/bouffalolab/bl_iot_sdk/blob/ee4a10b1a1e3609243bd5e7b3a45f02d768f6c14/components/bl602/bl602_std/bl602_std/StdDriver/Inc/bl602_glb.h#L286-L297
    xtal_type: u8,
    /// The PLL output clock type
    // TODO: Create enum type
    // https://github.com/bouffalolab/bl_iot_sdk/blob/ee4a10b1a1e3609243bd5e7b3a45f02d768f6c14/components/bl602/bl602_std/bl602_std/StdDriver/Inc/bl602_glb.h#L299-L312
    pll_clock: u8,
    /// HCLK divider
    hclk_divider: u8,
    /// BCLK divider
    bclk_divider: u8,
    /// Flash clock type
    // TODO: Create enum type
    // https://github.com/bouffalolab/bl_iot_sdk/blob/ee4a10b1a1e3609243bd5e7b3a45f02d768f6c14/components/bl602/bl602_std/bl602_std/StdDriver/Inc/bl602_glb.h#L101-L111
    flash_clock_type: u8,
    /// Flash clock divider
    flash_clock_divider: u8,
    /// CRC32 checksum
    crc32: u32,
}

#[derive(Debug, Copy, Default, Clone, Eq, PartialEq)]
pub struct FlashConfig {
    // Serail flash interface mode,bit0-3:IF mode,bit4:unwrap */
    io_mode: u8,
    // Support continuous read mode,bit0:continuous read mode support,bit1:read mode cfg
    continuous_read_support: u8,
    // SPI clock delay,bit0-3:delay,bit4-6:pad delay
    clock_delay: u8,
    // SPI clock phase invert,bit0:clck invert,bit1:rx invert,bit2-4:pad delay,bit5-7:pad delay */
    clock_invert: u8,
    // Flash enable reset command */
    reset_enable_cmd: u8,
    // Flash reset command */
    reset_cmd: u8,
    // Flash reset continuous read command */
    reset_continuous_read_cmd: u8,
    // Flash reset continuous read command size */
    reset_continuous_read_cmd_size: u8,
    // JEDEC ID command */
    jedec_id_cmd: u8,
    // JEDEC ID command dummy clock */
    jedec_id_cmd_dummy_clock: u8,
    // QPI JEDEC ID comamnd */
    qpi_jedec_id_cmd: u8,
    // QPI JEDEC ID command dummy clock */
    qpi_jedec_id_cmd_dummy_clock: u8,
    // Sector size - 1024 bytes
    sector_size: u8,
    // Manufacturer ID
    manufacturer_id: u8,
    // Page size
    page_size: u16,
    // Chip erase command
    chip_erase_cmd: u8,
    // Sector erase command
    sector_erase_cmd: u8,
    // Block 32K erase command,some Micron not support */
    block_erase_32k_cmd: u8,
    // Block 64K erase command */
    block_erase_64k_cmd: u8,
    // Need before every erase or program */
    write_enable_cmd: u8,
    // Page program cmd */
    page_program_cmd: u8,
    // QIO page program cmd */
    qio_page_program_cmd: u8,
    // QIO page program address mode */
    qio_page_program_address_mode: u8,
    // Fast read command */
    fast_read_cmd: u8,
    // Fast read command dummy clock */
    fast_read_cmd_dummy_clock: u8,
    // QPI fast read command */
    qpi_fast_read_cmd: u8,
    // QPI fast read command dummy clock */
    qpi_fast_read_cmd_dummy_clock: u8,
    // Fast read dual output command */
    fast_read_dual_output_cmd: u8,
    // Fast read dual output command dummy clock */
    fast_read_dual_output_cmd_dummy_clock: u8,
    // Fast read dual io comamnd */
    fast_read_dual_io_cmd: u8,
    // Fast read dual io command dummy clock */
    fast_read_dual_io_cmd_dummy_clock: u8,
    // Fast read quad output comamnd */
    fast_read_quad_output_cmd: u8,
    // Fast read quad output comamnd dummy clock */
    fast_read_quad_output_cmd_dummy_clock: u8,
    // Fast read quad io comamnd */
    fast_read_quad_io_cmd: u8,
    // Fast read quad io comamnd dummy clock */
    fast_read_quad_io_cmd_dummy_clock: u8,
    // QPI fast read quad io comamnd */
    qpi_fast_read_quad_io_cmd: u8,
    // QPI fast read QIO dummy clock */
    qpi_fast_read_quad_io_cmd_dummy_clock: u8,
    // QPI program command */
    qpi_program_cmd: u8,
    // Enable write reg */
    // writeVregEnableCmd
    volatile_register_write_enable_cmd: u8,
    // Write enable register index */
    write_enable_reg_index: u8,
    // Quad mode enable register index */
    quad_mode_enable_reg_index: u8,
    // Busy status register index */
    busy_status_reg_index: u8,
    // Write enable bit pos */
    write_enable_bit_pos: u8,
    // Quad enable bit pos */
    quad_enable_bit_pos: u8,
    // Busy status bit pos */
    busy_status_bit_pos: u8,
    // Register length of write enable */
    write_enable_reg_write_len: u8,
    // Register length of write enable status */
    write_enable_reg_read_len: u8,
    // Register length of contain quad enable */
    quad_enable_reg_write_len: u8,
    // Register length of contain quad enable status */
    quad_enable_reg_read_len: u8,
    // Release power down command */
    release_power_down_cmd: u8,
    // Register length of contain busy status */
    busy_status_reg_read_len: u8,
    // Read register command buffer */
    read_reg_cmd_buffer: [u8; 4],
    // Write register command buffer */
    write_reg_cmd_buffer: [u8; 4],
    // Enter qpi command */
    enter_qpi_cmd: u8,
    // Exit qpi command */
    exit_qpi_cmd: u8,
    // Config data for continuous read mode */
    continuous_read_mode_cfg: u8,
    // Config data for exit continuous read mode */
    continuous_read_mode_exit_cfg: u8,
    // Enable burst wrap command */
    enable_burst_wrap_cmd: u8,
    // Enable burst wrap command dummy clock */
    enable_burst_wrap_cmd_dummy_clock: u8,
    // Data and address mode for this command */
    burst_wrap_data_mode: u8,
    // Data to enable burst wrap */
    burst_wrap_data: u8,
    // Disable burst wrap command */
    disable_burst_wrap_cmd: u8,
    // Disable burst wrap command dummy clock */
    disable_burst_wrap_cmd_dummy_clock: u8,
    // Data and address mode for this command */
    disable_burst_wrap_data_mode: u8,
    // Data to disable burst wrap */
    disable_burst_wrap_data: u8,
    // 4K erase time */
    sector_erase_time_4k: u16,
    // 32K erase time */
    sector_erase_time_32k: u16,
    // 64K erase time */
    sector_erase_time_64k: u16,
    // Page program time */
    page_program_time: u16,
    // Chip erase time in ms */
    chip_erase_time: u16,
    // Release power down command delay time for wake up */
    power_down_delay: u8,
    // QE set data */
    quad_enable_data: u8,
}

impl Firmware {
    pub fn from_reader<R: ReadBytesExt + Seek>(mut reader: R) -> Result<Self, ParseError> {
        let mut magic = [0u8; 4];

        // Read the magic header
        reader.read_exact(&mut magic)?;

        // Determine which CPU this firmware is for
        let cpu = match &magic {
            b"BFNP" => Cpu::Cpu0,
            b"BFAP" => Cpu::Cpu1,
            _ => {
                return Err(ParseError::BootHeaderError(
                    BootHeaderError::InvalidMagicHeader(magic),
                ))
            }
        };

        // Read the boot header revision
        let revision = reader.read_u32::<LittleEndian>()?;

        // Skip the flash config
        reader.seek(SeekFrom::Current(0x5c))?;

        // Read the flash config
        let clock_config = ClockConfig::from_reader(&mut reader)?;

        // Read the boot flags
        let boot_config = reader.read_u32::<LittleEndian>()?;

        // Read the image segment info
        let image_segment_info = reader.read_u32::<LittleEndian>()?;

        // Read the entry point
        let entry_point = reader.read_u32::<LittleEndian>()?;

        // Read the image start offset
        let image_start = reader.read_u32::<LittleEndian>()?;

        // Read the image hash
        let mut hash = [0u8; 32];
        reader.read_exact(&mut hash)?;

        // Skip the 8 reserved, unused bytes
        reader.seek(SeekFrom::Current(8))?;

        // Read the crc32 checksum
        let crc32 = reader.read_u32::<LittleEndian>()?;

        Ok(Firmware {
            cpu,
            revision,
            flash_config: FlashConfig::default(),
            clock_config,
            boot_config,
            image_segment_info,
            entry_point,
            image_start,
            hash,
            crc32,
        })
    }
}

impl FlashConfig {
    pub fn from_reader<R: ReadBytesExt + Seek>(reader: &mut R) -> Result<Self, ParseError> {
        Ok(FlashConfig::default())
    }
}

impl ClockConfig {
    pub fn from_reader<R: ReadBytesExt + Seek>(reader: &mut R) -> Result<Self, ParseError> {
        let mut conf = ClockConfig::default();
        let mut magic = [0u8; 4];

        // Read the magic header
        reader.read_exact(&mut magic)?;

        // Assert that the magic header is correct
        // Currently disabled because the eflash loaders have a magic header of [0, 0, 0, 0]
        //
        if &magic != b"PCFG" {
            return Err(ParseError::ClockConfigError(
                ClockConfigError::InvalidMagicHeader(magic),
            ));
        }

        // Read the xtal type
        conf.xtal_type = reader.read_u8()?;

        // Read the PLL clock
        conf.pll_clock = reader.read_u8()?;

        // Read the HCLK divider
        conf.hclk_divider = reader.read_u8()?;

        // Read the BCLK divider
        conf.bclk_divider = reader.read_u8()?;

        // Read the flash clock type
        conf.flash_clock_type = reader.read_u8()?;

        // Read the flash clock divider
        conf.flash_clock_divider = reader.read_u8()?;

        // Skip the 2 reserved bytes that are currently unused
        reader.seek(SeekFrom::Current(2))?;

        // Read the CRC32 checksum
        conf.crc32 = reader.read_u32::<LittleEndian>()?;

        Ok(conf)
    }
}

pub struct FirmwareBuilder {
    /// The entry point of the firmware image
    entry_point: Option<u32>,
    /// Flash configuration
    flash_config: Option<FlashConfig>,
}

impl FirmwareBuilder {
    /// Sets the firmwares entry point to `entry_point`
    pub fn entry_point(&mut self, entry_point: u32) -> &mut FirmwareBuilder {
        self.entry_point = Some(entry_point);
        self
    }

    /// Sets the flash config to `flash_config`
    pub fn flash_config(&mut self, flash_config: FlashConfig) -> &mut FirmwareBuilder {
        self.flash_config = Some(flash_config);
        self
    }

    /// Builds the final Firmware from this FirmwareBuilder
    ///
    /// Returns the Firmware instance on success, a BuilderError otherwise
    pub fn build(&self) -> Result<Firmware, BuilderError> {
        let entry_point = self.entry_point.unwrap_or(DEFAULT_ENTRY_POINT);

        // Assert that a flash configuration has been set
        let flash_config = match self.flash_config {
            Some(flash_config) => flash_config,
            None => return Err(BuilderError::MissingFlashConfig),
        };

        let clock_config = ClockConfig::default();
        let boot_config = 0;

        Ok(Firmware {
            cpu: Cpu::Cpu0,
            revision: 1,
            flash_config,
            clock_config,
            boot_config,
            image_segment_info: 0,
            entry_point,
            image_start: 0,
            hash: [0; 32],
            crc32: 0,
        })
    }
}

impl Default for FirmwareBuilder {
    fn default() -> FirmwareBuilder {
        FirmwareBuilder {
            entry_point: None,
            flash_config: None,
        }
    }
}

impl Firmware {
    pub fn builder() -> FirmwareBuilder {
        FirmwareBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const REFERENCE_FIRMWARE: &[u8] = include_bytes!("../../test/test_reference_firmware.bin");

    #[test]
    fn it_should_read_clock_config() {
        let mut cursor = Cursor::new(&REFERENCE_FIRMWARE[0x64..0x74]);
        let clock_config = ClockConfig::from_reader(&mut cursor).unwrap();

        assert_eq!(clock_config.xtal_type, 1);
        assert_eq!(clock_config.flash_clock_divider, 222);
    }

    #[test]
    fn it_should_read_firmware() {
        let hash: [u8; 32] = [
            0xDD, 0x11, 0x42, 0x8A, 0x2A, 0x77, 0x9F, 0xFA, 0xCD, 0xB8, 0xBC, 0xEF, 0x9C, 0xB6,
            0x4C, 0xA3, 0x0F, 0x15, 0xAC, 0x19, 0xF5, 0x0E, 0xF3, 0x64, 0x50, 0x3E, 0xB3, 0xE5,
            0x0E, 0x00, 0x00, 0x00,
        ];
        let mut cursor = Cursor::new(&REFERENCE_FIRMWARE);
        let firmware = Firmware::from_reader(&mut cursor).unwrap();

        assert_eq!(firmware.cpu, Cpu::Cpu0);
        assert_eq!(firmware.revision, 1);
        assert_eq!(firmware.boot_config, 256);
        assert_eq!(firmware.image_segment_info, 0);
        assert_eq!(firmware.entry_point, 0x1000000);
        assert_eq!(firmware.image_start, 0x96703322);
        assert_eq!(firmware.hash, hash);
        assert_eq!(firmware.crc32, 0x1000098);
    }
}
