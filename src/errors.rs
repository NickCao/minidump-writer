use crate::thread_info::Pid;
use goblin;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InitError {
    #[error("IO error")]
    FileError(#[from] std::io::Error),
    #[error("No auxv entry found")]
    NoAuxvEntryFound,
    #[error("crash thread does not reference principal mapping")]
    PrincipalMappingNotReferenced,
}

#[derive(Error, Debug)]
pub enum MapsReaderError {
    // parse_from_line()
    #[error("Map entry malformed: No {0} found")]
    MapEntryMalformed(&'static str),
    #[error("Couldn't parse address")]
    UnparsableInteger(#[from] std::num::ParseIntError),
    #[error("Linux gate location doesn't fit in the required integer type")]
    LinuxGateNotConvertable(#[from] std::num::TryFromIntError),

    // get_mmap()
    #[error("Not safe to open mapping")] // TODO: Add info
    NotSafeToOpenMapping,
    #[error("IO Error")]
    FileError(#[from] std::io::Error),
    #[error("Mmapped file empty or not an ELF file")]
    MmapSanityCheckFailed,
    #[error("Symlink does not match")] // TODO: add info
    SymlinkError,

    // handle_deleted_file_in_mapping()
    #[error("Couldn't parse as ELF file")]
    ELFParsingFailed(#[from] goblin::error::Error),
    #[error("No soname found")] // TODO: Add info
    NoSoName,
}

#[derive(Debug, Error)]
pub enum AuxvReaderError {
    #[error("Invalid auxv format")] // TODO: Add info
    InvalidFormat,
    #[error("IO Error")]
    FileError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum CpuInfoError {
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("Not all entries of /proc/cpuinfo found!")]
    NotAllProcEntriesFound,
}

#[derive(Error, Debug)]
pub enum ThreadInfoError {
    #[error("Index out of bounds: Got {0}, only have {1}")]
    IndexOutOfBounds(usize, usize),
    #[error("Invalid ppid or tgid")] // TODO: Add info
    InvalidPid,
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("Couldn't parse address")]
    UnparsableInteger(#[from] std::num::ParseIntError),
    #[error("nix::ptrace() error")]
    PtraceError(#[from] nix::Error),
}

#[derive(Debug, Error)]
pub enum CpuSetError {
    #[error("Couldn't read from file")]
    IOError(#[from] std::io::Error),
    #[error("Couldn't parse core from file")]
    UnparsableInteger(#[from] std::num::ParseIntError),
    #[error("Couldn't parse cores: {0}")]
    UnparsableCores(String),
}

#[derive(Debug, Error)]
pub enum DumperError {
    #[error("nix::ptrace() error")]
    PtraceError(#[from] nix::Error),
    #[error("Skipped thread {0} due to it being part of the seccomp sandbox's trusted code")]
    DetachSkippedThread(Pid),
    #[error("No threads left to suspend")]
    SuspendNoThreadsLeft,
    #[error("No mapping for stack pointer found")] // TODO: Add info
    NoStackPointerMapping,
    #[error("Failed slice conversion")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
    #[error("Couldn't parse as ELF file")]
    ELFParsingFailed(#[from] goblin::error::Error),
    #[error("No build-id found")] // TODO: Add info
    NoBuildIDFound,
    #[error("Not safe to open mapping")] // TODO: Add info
    NotSafeToOpenMapping,
    #[error("Failed integer conversion")]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error("Maps reader error")]
    MapsReaderError(#[from] MapsReaderError),
}

#[derive(Debug, Error)]
pub enum MemoryWriterError {
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("Failed integer conversion")]
    TryFromIntError(#[from] std::num::TryFromIntError),
}

#[derive(Debug, Error)]
pub enum SectionAppMemoryError {
    #[error("Failed integer conversion")]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error("Failed to copy memory from process")]
    CopyFromProcessError(#[from] DumperError),
    #[error("Failed write memory")]
    MemoryWriterError(#[from] MemoryWriterError),
}

#[derive(Debug, Error)]
pub enum SectionExceptionStreamError {
    #[error("Failed write memory")]
    MemoryWriterError(#[from] MemoryWriterError),
}

#[derive(Debug, Error)]
pub enum SectionMappingsError {
    #[error("Failed write memory")]
    MemoryWriterError(#[from] MemoryWriterError),
    #[error("Failed to get effective path of mapping")]
    GetEffectivePathError(#[from] MapsReaderError),
}

#[derive(Debug, Error)]
pub enum SectionMemListError {
    #[error("Failed write memory")]
    MemoryWriterError(#[from] MemoryWriterError),
}

#[derive(Debug, Error)]
pub enum SectionSystemInfoError {
    #[error("Failed write memory")]
    MemoryWriterError(#[from] MemoryWriterError),
    #[error("Failed to get CPU Info")]
    CpuInfoError(#[from] CpuInfoError),
}

#[derive(Debug, Error)]
pub enum SectionThreadListError {
    #[error("Failed write memory")]
    MemoryWriterError(#[from] MemoryWriterError),
    #[error("Failed integer conversion")]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error("Failed to copy memory from process")]
    CopyFromProcessError(#[from] DumperError),
    #[error("Failed to get thread info")]
    ThreadInfoError(#[from] ThreadInfoError),
    #[error("Failed to write to buffer")]
    IOError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum SectionDsoDebugError {
    #[error("Failed to write to memory")]
    MemoryWriterError(#[from] MemoryWriterError),
    #[error("Could not find: {0}")] // TODO: Add info
    CouldNotFind(&'static str),
    #[error("Failed to copy memory from process")]
    CopyFromProcessError(#[from] DumperError),
    #[error("Failed to copy memory from process")]
    FromUTF8Error(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, Error)]
pub enum FileWriterError {
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("Failed to write to memory")]
    MemoryWriterError(#[from] MemoryWriterError),
}

#[derive(Debug, Error)]
pub enum WriterError {
    #[error("Error during init phase")]
    InitError(#[from] InitError),
    #[error(transparent)]
    DumperError(#[from] DumperError),
    #[error("Failed when writing section AppMemory")]
    SectionAppMemoryError(#[from] SectionAppMemoryError),
    #[error("Failed when writing section ExceptionStream")]
    SectionExceptionStreamError(#[from] SectionExceptionStreamError),
    #[error("Failed when writing section MappingsError")]
    SectionMappingsError(#[from] SectionMappingsError),
    #[error("Failed when writing section MemList")]
    SectionMemListError(#[from] SectionMemListError),
    #[error("Failed when writing section SystemInfo")]
    SectionSystemInfoError(#[from] SectionSystemInfoError),
    #[error("Failed when writing section ThreadList")]
    SectionThreadListError(#[from] SectionThreadListError),
    #[error("Failed when writing section DsoDebug")]
    SectionDsoDebugError(#[from] SectionDsoDebugError),
    #[error("Failed to write to memory")]
    MemoryWriterError(#[from] MemoryWriterError),
    #[error("Failed to write to file")]
    FileWriterError(#[from] FileWriterError),
    #[error("Failed to get current timestamp when writing header of minidump")]
    SystemTimeError(#[from] std::time::SystemTimeError),
}
