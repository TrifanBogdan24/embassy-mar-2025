use sysinfo;

pub async fn init() -> sysinfo::System {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
    sys.refresh_cpu_all();

    sys
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Kind {
    // For using only lower-case letters in the URL

    #[serde(rename = "system")]
    System,
    #[serde(rename = "process")]
    Process,
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "cpu")]
    Cpu,
    #[serde(rename = "disk")]
    Disk,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct System {
    name: String,
    kernel_version: String,
    os_version: String,
    host_name: String,
    uptime: u64,
}

impl System {
    pub fn generate() -> Self {
        Self {
            name: sysinfo::System::name().unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: sysinfo::System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            os_version: sysinfo::System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            host_name: sysinfo::System::host_name().unwrap_or_else(|| "Unknown".to_string()),
            uptime: sysinfo::System::uptime(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Process {
    pid: u32,
    name: String,
    memory: u64,
    cpu_usage: f32,
    run_time: u64,
}

impl Process {
    pub fn generate(sys: &sysinfo::System) -> Vec<Self> {
        sys.processes()
            .iter()
            .map(|(&pid, proc_)| Self {
                pid: pid.as_u32(),
                name: proc_.name().to_string_lossy().to_string(),
                memory: proc_.memory(),
                cpu_usage: proc_.cpu_usage(),
                run_time: proc_.run_time(),
            })
            .collect()
    }
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Memory {
    used: u64,
    total: u64,
}

impl Memory {
    pub fn generate(sys: &sysinfo::System) -> Self {
        Self {
            used: sys.used_memory(),
            total: sys.total_memory(),
        }
    }
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoreMetrics {
    name: String,
    brand: String,
    usage: f32,
    frequency: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cpu {
    cpu_usage: f32,
    cores: Vec<CoreMetrics>,
}

impl Cpu {
    pub fn generate(sys: &sysinfo::System) -> Self {
        let cores = sys.cpus().iter().map(|cpu| CoreMetrics {
            name: cpu.name().to_string(),
            brand: cpu.brand().to_string(),
            usage: cpu.cpu_usage(),
            frequency: cpu.frequency(),
        }).collect();

        Self {
            cpu_usage: sys.global_cpu_usage(),
            cores,
        }
    }
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Disk {
    name: String,
    available_space: u64,
    total_space: u64,
    is_removable: bool,
}

impl Disk {
    pub fn generate(_sys: &sysinfo::System) -> Vec<Self> {
        // TODO implement it
        vec![]
    }
}



#[derive(serde::Serialize, serde::Deserialize)]
pub struct Summary {
    system: System,
    process: Vec<Process>,
    memory: Memory,
    cpu: Cpu,
    disk: Vec<Disk>,
}

impl Summary {
    pub fn generate(sys: &mut sysinfo::System) -> Self {
        Self {
            system: System::generate(),
            process: Process::generate(sys),
            memory: Memory::generate(sys),
            cpu: Cpu::generate(sys),
            disk: Disk::generate(sys),
        }
    }
}
