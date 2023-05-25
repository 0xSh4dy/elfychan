pub struct elfychan_ident{
    pub ei_class:u8,
    pub ei_data:u8,
    pub ei_version:u8,
    pub ei_osabi:u8,
    pub ei_abiversion:u8
}

pub struct elfychan_new_header{
    pub etype:u16,
    pub emachine:u16,
    pub eversion:u32,
    pub eentry:u64,
    pub ephoff:u64,
    pub eshoff:u64,
    pub eflags:u32,
    pub eehsize:u16,
    pub ephentsize:u16,
    pub ephnum:u16,
    pub eshentsize:u16,
    pub eshnum:u16,
    pub eshstrndx:u16
}

impl elfychan_ident{
    pub fn parse_class(&self){
        match self.ei_class{
            1=>println!("Class -> ELF 32-bit"),
            2=>println!("Class -> ELF 64-bit"),
            _=>println!("Class -> Unknown class {}",self.ei_class)
        }
    }
    pub fn parse_data(&self){
        match self.ei_data{
            1=>println!("Encoding -> Little endian"),
            2=>println!("Encoding -> Big endian"),
            _=>println!("Encoding -> Invalid format")
        }
    }
    pub fn parse_version(&self){
        if self.ei_version<=0{
            println!("Version no. -> Invalid version");
        }
        else{
            println!("Version no.-> Current");
        }
    }
    pub fn parse_abi(&self){
        match self.ei_osabi{
            0=>println!("ABI -> ELFOSABI_SYSV"),
            1=>println!("ABI -> ELFOSABI_HPUX"),
            2=>println!("ABI -> ELFOSABI_NETBSD"),
            3=>println!("ABI -> ELFOSABI_GNU"),
            4=>println!("ABI -> ELFOSABI_HURD"),
            6=>println!("ABI -> ELFOSABI_SOLARIS"),
            7=>println!("ABI -> ELFOSABI_AIX"),
            8=>println!("ABI -> ELFOSABI_IRIX"),
            9=>println!("ABI -> ELFOSABI_FREEBSD"),
            10=>println!("ABI -> ELFOSABI_TRU64"),
            11=>println!("ABI -> ELFOSABI_MODESTO"),
            12=>println!("ABI -> ELFOSABI_OPENBSD"),
            14=>println!("ABI -> ELFOSABI_NSK"),
            64=>println!("ABI -> ELFOSABI_ARM_AEABI"),
            97=>println!("ABI -> ELFOSABI_ARM"),
            255=>println!("ABI -> ELFOSABI_STANDALONE"),
            _=>{}
        }
    }
}

impl elfychan_new_header{
    pub fn parse_e_type(&self){
        match self.etype{
            0=>println!("Type -> ET_NONE (No type)"),
            1=>println!("Type -> ET_REL (Relocatable file)"),
            2=>println!("Type -> ET_EXEC (Executable file)"),
            3=>println!("Type -> ET_DYN (Shared object file)"),
            4=>println!("Type -> ET_CORE(Core file)"),
            _=>println!("Type -> Unknown")
        }
    }
    pub fn parse_e_machine(&self){
        match self.emachine{
            0=>println!("Machine -> EM_NONE (No machine)"),
            1=>println!("Machine -> EM_M32 (AT&T WE 32100)"),
            2=>println!("Machine -> EM_SPARC (SPARC)"),
            3=>println!("Machine -> EM_386 (Intel 80386)"),
            4=>println!("Machine -> EM_68K (Motorola 68000)"),
            5=>println!("Machine -> EM_88K (Motorola 88000)"),
            6=>println!("Machine -> EM_IAMCU (Intel MCU)"),
            7=>println!("Machine -> EM_860 (Intel 80860)"),
            8=>println!("Machine -> EM_MIPS (MIPS I Architecture)"),
            9=>println!("Machine -> EM_S370 (IBM System/370 Processor)"),
            10=>println!("Machine -> EM_MIPS_RS3_LE (MIPS RS3000 Little-endian)"),
            15=>println!("Machine -> EM_PARISC (Hewlett-Packard PA-RISC)"),
            17=>println!("Machine -> EM_VPP500 (Fujitsu VPP500)"),
            18=>println!("Machine -> EM_SPARC32PLUS (Enhanced instruction set SPARC)"),
            19=>println!("Machine -> EM_960 (Intel 80960)"),
            20=>println!("Machine -> EM_PPC (PowerPC)"),
            21=>println!("Machine -> EM_PPC64 (64-bit PowerPC)"),
            22=>println!("Machine -> EM_S390 (IBM System/390 Processor)"),
            23=>println!("Machine -> EM_SPU (IBM SPU/SPC)"),
            36=>println!("Machine -> EM_V800 (NEC V800)"),
            37=>println!("Machine -> EM_FR20 (Fujitsu FR20)"),
            38=>println!("Machine -> EM_RH32 (TRW RH-32)"),
            39=>println!("Machine -> EM_RCE (Motorola RCE)"),
            40=>println!("Machine -> EM_ARM (ARM 32-bit architecture (AARCH32))"),
            41=>println!("Machine -> EM_ALPHA (Digital Alpha)"),
            42=>println!("Machine -> EM_SH (Hitachi SH)"),
            43=>println!("Machine -> EM_SPARCV9 (SPARC Version 9)"),
            44=>println!("Machine -> EM_TRICORE (Siemens TriCore embedded processor)"),
            45=>println!("Machine -> EM_ARC (Argonaut RISC Core, Argonaut Technologies Inc.)"),
            46=>println!("Machine -> EM_H8_300 (Hitachi H8/300)"),
            47=>println!("Machine -> EM_H8_300H (Hitachi H8/300H)"),
            48=>println!("Machine -> EM_H8S (Hitachi H8S)"),
            49=>println!("Machine -> EM_H8_500 (Hitachi H8/500)"),
            50=>println!("Machine -> EM_IA_64 (Intel IA-64 processor architecture)"),
            51=>println!("Machine -> EM_MIPS_X (Stanford MIPS-X)"),
            52=>println!("Machine -> EM_COLDFIRE (Motorola ColdFire)"),
            53=>println!("Machine -> EM_68HC12 (Motorola M68HC12)"),
            54=>println!("Machine -> EM_MMA (Fujitsu MMA Multimedia Accelerator)"),
            55=>println!("Machine -> EM_PCP (Siemens PCP)"),
            56=>println!("Machine -> EM_NCPU (Sony nCPU embedded RISC processor)"),
            57=>println!("Machine -> EM_NDR1 (Denso NDR1 microprocessor)"),
            58=>println!("Machine -> EM_STARCORE (Motorola Star*Core processor)"),
            59=>println!("Machine -> EM_ME16 (Toyota ME16 processor)"),
            60=>println!("Machine -> EM_ST100 (STMicroelectronics ST100 processor)"),
            61=>println!("Machine -> EM_TINYJ (Advanced Logic Corp. TinyJ embedded processor family)"),
            62=>println!("Machine -> EM_X86_64 (AMD x86-64 architecture)"),
            63=>println!("Machine -> EM_PDSP (Sony DSP Processor)"),
            64=>println!("Machine -> EM_PDP10 (Digital Equipment Corp. PDP-10)"),
            65=>println!("Machine -> EM_PDP11 (Digital Equipment Corp. PDP-11)"),
            66=>println!("Machine -> EM_FX66 (Siemens FX66 microcontroller)"),
            67=>println!("Machine -> EM_ST9PLUS (STMicroelectronics ST9+ 8/16 bit microcontroller)"),
            68=>println!("Machine -> EM_ST7 (STMicroelectronics ST7 8-bit microcontroller)"),
            69=>println!("Machine -> EM_68HC16 (Motorola MC68HC16 Microcontroller)"),
            70=>println!("Machine -> EM_68HC11 (Motorola MC68HC11 Microcontroller)"),
            71=>println!("Machine -> EM_68HC08 (Motorola MC68HC08 Microcontroller)"),
            72=>println!("Machine -> EM_68HC05 (Motorola MC68HC05 Microcontroller)"),
            73=>println!("Machine -> EM_SVX (Silicon Graphics SVx)"),
            74=>println!("Machine -> EM_ST19 (STMicroelectronics ST19 8-bit microcontroller)"),
            75=>println!("Machine -> EM_VAX (Digital VAX)"),
            76=>println!("Machine -> EM_CRIS (Axis Communications 32-bit embedded processor)"),
            77=>println!("Machine -> EM_JAVELIN (Infineon Technologies 32-bit embedded processor)"),
            78=>println!("Machine -> EM_FIREPATH (Element 14 64-bit DSP Processor)"),
            79=>println!("Machine -> EM_ZSP (LSI Logic 16-bit DSP Processor)"),
            80=>println!("Machine -> EM_MMIX (Donald Knuth's educational 64-bit processor)"),
            81=>println!("Machine -> EM_HUANY (Harvard University machine-independent object files)"),
            82=>println!("Machine -> EM_PRISM (SiTera Prism)"),
            83=>println!("Machine -> EM_AVR (Atmel AVR 8-bit microcontroller)"),
            84=>println!("Machine -> EM_FR30 (Fujitsu FR30)"),
            85=>println!("Machine -> EM_D10V (Mitsubishi D10V)"),
            86=>println!("Machine -> EM_D30V (Mitsubishi D30V)"),
            87=>println!("Machine -> EM_V850 (NEC v850)"),
            88=>println!("Machine -> EM_M32R (Mitsubishi M32R)"),
            89=>println!("Machine -> EM_MN10300 (Matsushita MN10300)"),
            90=>println!("Machine -> EM_MN10200 (Matsushita MN10200)"),
            91=>println!("Machine -> EM_PJ (picoJava)"),
            92=>println!("Machine -> EM_OPENRISC (OpenRISC 32-bit embedded processor)"),
            93=>println!("Machine -> EM_ARC_COMPACT (ARC International ARCompact processor (old spelling/synonym: EM_ARC_A5))"),
            94=>println!("Machine -> EM_XTENSA (Tensilica Xtensa Architecture)"),
            95=>println!("Machine -> EM_VIDEOCORE (Alphamosaic VideoCore processor)"),
            96=>println!("Machine -> EM_TMM_GPP (Thompson Multimedia General Purpose Processor)"),
            97=>println!("Machine -> EM_NS32K (National Semiconductor 32000 series)"),
            98=>println!("Machine -> EM_TPC (Tenor Network TPC processor)"),
            99=>println!("Machine -> EM_SNP1K (Trebia SNP 1000 processor)"),
            100=>println!("Machine -> EM_ST200 (STMicroelectronics (www.st.com) ST200 microcontroller)"),
            101=>println!("Machine -> EM_IP2K (Ubicom IP2xxx microcontroller family)"),
            102=>println!("Machine -> EM_MAX (MAX Processor)"),
            103=>println!("Machine -> EM_CR (National Semiconductor CompactRISC microprocessor)"),
            104=>println!("Machine -> EM_F2MC16 (Fujitsu F2MC16)"),
            105=>println!("Machine -> EM_MSP430 (Texas Instruments embedded microcontroller msp430)"),
            106=>println!("Machine -> EM_BLACKFIN (Analog Devices Blackfin (DSP) processor)"),
            107=>println!("Machine -> EM_SE_C33 (S1C33 Family of Seiko Epson processors)"),
            108=>println!("Machine -> EM_SEP (Sharp embedded microprocessor)"),
            109=>println!("Machine -> EM_ARCA (Arca RISC Microprocessor)"),
            110=>println!("Machine -> EM_UNICORE (Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University)"),
            111=>println!("Machine -> EM_EXCESS (eXcess: 16/32/64-bit configurable embedded CPU)"),
            112=>println!("Machine -> EM_DXP (Icera Semiconductor Inc. Deep Execution Processor)"),
            113=>println!("Machine -> EM_ALTERA_NIOS2 (Altera Nios II soft-core processor)"),
            114=>println!("Machine -> EM_CRX (National Semiconductor CompactRISC CRX microprocessor)"),
            115=>println!("Machine -> EM_XGATE (Motorola XGATE embedded processor)"),
            116=>println!("Machine -> EM_C166 (Infineon C16x/XC16x processor)"),
            117=>println!("Machine -> EM_M16C (Renesas M16C series microprocessors)"),
            118=>println!("Machine -> EM_DSPIC30F (Microchip Technology dsPIC30F Digital Signal Controller)"),
            119=>println!("Machine -> EM_CE (Freescale Communication Engine RISC core)"),
            120=>println!("Machine -> EM_M32C (Renesas M32C series microprocessors)"),
            131=>println!("Machine -> EM_TSK3000 (Altium TSK3000 core)"),
            132=>println!("Machine -> EM_RS08 (Freescale RS08 embedded processor)"),
            133=>println!("Machine -> EM_SHARC (Analog Devices SHARC family of 32-bit DSP processors)"),
            134=>println!("Machine -> EM_ECOG2 (Cyan Technology eCOG2 microprocessor)"),
            135=>println!("Machine -> EM_SCORE7 (Sunplus S+core7 RISC processor)"),
            136=>println!("Machine -> EM_DSP24 (New Japan Radio (NJR) 24-bit DSP Processor)"),
            137=>println!("Machine -> EM_VIDEOCORE3 (Broadcom VideoCore III processor)"),
            138=>println!("Machine -> EM_LATTICEMICO32 (RISC processor for Lattice FPGA architecture)"),
            139=>println!("Machine -> EM_SE_C17 (Seiko Epson C17 family)"),
            140=>println!("Machine -> EM_TI_C6000 (The Texas Instruments TMS320C6000 DSP family)"),
            141=>println!("Machine -> EM_TI_C2000 (The Texas Instruments TMS320C2000 DSP family)"),
            142=>println!("Machine -> EM_TI_C5500 (The Texas Instruments TMS320C55x DSP family)"),
            143=>println!("Machine -> EM_TI_ARP32 (Texas Instruments Application Specific RISC Processor, 32bit fetch)"),
            144=>println!("Machine -> EM_TI_PRU (Texas Instruments Programmable Realtime Unit)"),
            160=>println!("Machine -> EM_MMDSP_PLUS (STMicroelectronics 64bit VLIW Data Signal Processor)"),
            161=>println!("Machine -> EM_CYPRESS_M8C (Cypress M8C microprocessor)"),
            162=>println!("Machine -> EM_R32C (Renesas R32C series microprocessors)"),
            163=>println!("Machine -> EM_TRIMEDIA (NXP Semiconductors TriMedia architecture family)"),
            164=>println!("Machine -> EM_QDSP6 (QUALCOMM DSP6 Processor)"),
            165=>println!("Machine -> EM_8051 (Intel 8051 and variants)"),
            166=>println!("Machine -> EM_STXP7X (STMicroelectronics STxP7x family of configurable and extensible RISC processors)"),
            167=>println!("Machine -> EM_NDS32 (Andes Technology compact code size embedded RISC processor family)"),
            168=>println!("Machine -> EM_ECOG1 (Cyan Technology eCOG1X family)"),
            169=>println!("Machine -> EM_MAXQ30 (Dallas Semiconductor MAXQ30 Core Micro-controllers)"),
            170=>println!("Machine -> EM_XIMO16 (New Japan Radio (NJR) 16-bit DSP Processor)"),
            171=>println!("Machine -> EM_MANIK (M2000 Reconfigurable RISC Microprocessor)"),
            172=>println!("Machine -> EM_CRAYNV2 (Cray Inc. NV2 vector architecture)"),
            173=>println!("Machine -> EM_RX (Renesas RX family)"),
            174=>println!("Machine -> EM_METAG (Imagination Technologies META processor architecture)"),
            175=>println!("Machine -> EM_MCST_ELBRUS (MCST Elbrus general purpose hardware architecture)"),
            176=>println!("Machine -> EM_ECOG16 (Cyan Technology eCOG16 family)"),
            177=>println!("Machine -> EM_CR16 (National Semiconductor CompactRISC CR16 16-bit microprocessor)"),
            178=>println!("Machine -> EM_ETPU (Freescale Extended Time Processing Unit)"),
            179=>println!("Machine -> EM_SLE9X (Infineon Technologies SLE9X core)"),
            180=>println!("Machine -> EM_L10M (Intel L10M)"),
            181=>println!("Machine -> EM_K10M (Intel K10M)"),
            183=>println!("Machine -> EM_AARCH64 (ARM 64-bit architecture (AARCH64))"),
            185=>println!("Machine -> EM_AVR32 (Atmel Corporation 32-bit microprocessor family)"),
            186=>println!("Machine -> EM_STM8 (STMicroeletronics STM8 8-bit microcontroller)"),
            187=>println!("Machine -> EM_TILE64 (Tilera TILE64 multicore architecture family)"),
            188=>println!("Machine -> EM_TILEPRO (Tilera TILEPro multicore architecture family)"),
            189=>println!("Machine -> EM_MICROBLAZE (Xilinx MicroBlaze 32-bit RISC soft processor core)"),
            190=>println!("Machine -> EM_CUDA (NVIDIA CUDA architecture)"),
            191=>println!("Machine -> EM_TILEGX (Tilera TILE-Gx multicore architecture family)"),
            192=>println!("Machine -> EM_CLOUDSHIELD (CloudShield architecture family)"),
            193=>println!("Machine -> EM_COREA_1ST (KIPO-KAIST Core-A 1st generation processor family)"),
            194=>println!("Machine -> EM_COREA_2ND (KIPO-KAIST Core-A 2nd generation processor family)"),
            195=>println!("Machine -> EM_ARC_COMPACT2 (Synopsys ARCompact V2)"),
            196=>println!("Machine -> EM_OPEN8 (Open8 8-bit RISC soft processor core)"),
            197=>println!("Machine -> EM_RL78 (Renesas RL78 family)"),
            198=>println!("Machine -> EM_VIDEOCORE5 (Broadcom VideoCore V processor)"),
            199=>println!("Machine -> EM_78KOR (Renesas 78KOR family)"),
            200=>println!("Machine -> EM_56800EX (Freescale 56800EX Digital Signal Controller (DSC))"),
            201=>println!("Machine -> EM_BA1 (Beyond BA1 CPU architecture)"),
            202=>println!("Machine -> EM_BA2 (Beyond BA2 CPU architecture)"),
            203=>println!("Machine -> EM_XCORE (XMOS xCORE processor family)"),
            204=>println!("Machine -> EM_MCHP_PIC (Microchip 8-bit PIC(r) family)"),
            205=>println!("Machine -> EM_INTEL205 (Reserved by Intel)"),
            206=>println!("Machine -> EM_INTEL206 (Reserved by Intel)"),
            207=>println!("Machine -> EM_INTEL207 (Reserved by Intel)"),
            208=>println!("Machine -> EM_INTEL208 (Reserved by Intel)"),
            209=>println!("Machine -> EM_INTEL209 (Reserved by Intel)"),
            210=>println!("Machine -> EM_KM32 (KM211 KM32 32-bit processor)"),
            211=>println!("Machine -> EM_KMX32 (KM211 KMX32 32-bit processor)"),
            212=>println!("Machine -> EM_KMX16 (KM211 KMX16 16-bit processor)"),
            213=>println!("Machine -> EM_KMX8 (KM211 KMX8 8-bit processor)"),
            214=>println!("Machine -> EM_KVARC (KM211 KVARC processor)"),
            215=>println!("Machine -> EM_CDP (Paneve CDP architecture family)"),
            216=>println!("Machine -> EM_COGE (Cognitive Smart Memory Processor)"),
            217=>println!("Machine -> EM_COOL (Bluechip Systems CoolEngine)"),
            218=>println!("Machine -> EM_NORC (Nanoradio Optimized RISC)"),
            219=>println!("Machine -> EM_CSR_KALIMBA (CSR Kalimba architecture family)"),
            220=>println!("Machine -> EM_Z80 (Zilog Z80)"),
            221=>println!("Machine -> EM_VISIUM (Controls and Data Services VISIUMcore processor)"),
            222=>println!("Machine -> EM_FT32 (FTDI Chip FT32 high performance 32-bit RISC architecture)"),
            223=>println!("Machine -> EM_MOXIE (Moxie processor family)"),
            224=>println!("Machine -> EM_AMDGPU (AMD GPU architecture)"),
            243=>println!("Machine -> EM_RISCV (RISC-V)"),
            _=>println!("Machine -> Not known")
        }
    }
    pub fn parse_e_version(&self){
        if self.eversion==0{
            println!("File Version -> Invalid")
        }
        else if self.eversion==1 {
            println!("File Version -> Current")
        }
        else{
            println!("File Version -> Unknown")
        }
    }
    pub fn parse_entrypoint(&self){
        println!("Entrypoint -> {:#x}", self.eentry);
    }
}