#[cfg(windows)]
use serde::Deserialize;

#[cfg(windows)]
use wmi::WMIConnection;

#[cfg(windows)]
#[derive(Deserialize)]
#[serde(rename = "MaxClockSpeed")]
struct Processor {
    max_clock_speed: u32,
}

pub fn get_maximum_frequency() -> anyhow::Result<Option<u64>> {
    #[cfg(target_os = "linux")]
    {
        let Ok(frequency) =
            std::fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
        else {
            return Ok(None);
        };

        let frequency = frequency.trim().parse::<u64>()?;

        return Ok(Some(frequency * 1000));
    }

    #[cfg(target_os = "windows")]
    {
        let wmi = WMIConnection::new()?;

        let processors: Vec<Processor> = wmi.query()?;

        let max_frequency = processors.iter().map(|cpu| cpu.max_clock_speed).max();

        return Ok(max_frequency.map(|mhz| mhz as u64 * 1_000_000));
    }

    #[cfg(target_os = "macos")]
    {
        let Ok(ctl) = sysctl::Ctl::new("hw.cpufrequency_max") else {
            return Ok(None);
        };

        let Ok(value) = ctl.value() else {
            return Ok(None);
        };

        let frequency = value.as_long().or_else(|| value.as_s64()).copied();

        return Ok(frequency.and_then(|hz| u64::try_from(hz).ok()));
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    Ok(None)
}
