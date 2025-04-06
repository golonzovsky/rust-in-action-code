use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

// Enum to represent the type of memory region: Mapped or Free
#[derive(Clone)]
enum RegionType {
  Mapped {
    perms: String,    // Permissions (e.g., "r-xp")
    offset: usize,    // Offset in the mapped file
    dev: String,      // Device (major:minor)
    inode: u64,       // Inode number
    pathname: String, // File path or mapping type (e.g., "[stack]")
  },
  Free,
}

// Struct to hold memory region information
struct MemoryRegion {
  start: usize,            // Start address of the region
  end: usize,              // End address of the region
  region_type: RegionType, // Type of the region
}

// Function to read mapped regions from /proc/self/maps
fn read_mapped_regions() -> Vec<MemoryRegion> {
  let mut mapped_regions = Vec::new();

  // Open /proc/self/maps, which contains memory mappings for the current process
  if let Ok(file) = File::open("/proc/self/maps") {
    let reader = BufReader::new(file);
    for line in reader.lines() {
      if let Ok(line) = line {
        // Split the line into whitespace-separated fields
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() >= 5 {
          // Parse address range (e.g., "00400000-00410000")
          let addr_range = fields[0];
          let perms = fields[1].to_string();
          let offset_str = fields[2];
          let dev = fields[3].to_string();
          let inode_str = fields[4];
          // Pathname may contain spaces, so join remaining fields
          let pathname = if fields.len() > 5 {
            fields[5..].join(" ")
          } else {
            "".to_string()
          };

          // Split and parse the start and end addresses
          let addrs: Vec<&str> = addr_range.split('-').collect();
          if addrs.len() == 2 {
            if let (Ok(start), Ok(end)) = (usize::from_str_radix(addrs[0], 16), usize::from_str_radix(addrs[1], 16)) {
              let offset = usize::from_str_radix(offset_str, 16).unwrap_or(0);
              let inode = inode_str.parse::<u64>().unwrap_or(0);
              mapped_regions.push(MemoryRegion {
                start,
                end,
                region_type: RegionType::Mapped {
                  perms,
                  offset,
                  dev,
                  inode,
                  pathname,
                },
              });
            }
          }
        }
      }
    }
  }

  // Sort regions by start address (though /proc/self/maps is usually sorted)
  mapped_regions.sort_by_key(|r| r.start);
  mapped_regions
}

// Function to generate all regions, including free regions between mapped ones
fn generate_all_regions(mapped_regions: &[MemoryRegion]) -> Vec<MemoryRegion> {
  let mut all_regions = Vec::new();
  let mut current_addr = 0; // Start from address 0

  for mapped in mapped_regions {
    // If there's a gap before this mapped region, add a free region
    if current_addr < mapped.start {
      all_regions.push(MemoryRegion {
        start: current_addr,
        end: mapped.start,
        region_type: RegionType::Free,
      });
    }
    // Add the mapped region
    all_regions.push(MemoryRegion {
      start: mapped.start,
      end: mapped.end,
      region_type: mapped.region_type.clone(),
    });
    current_addr = mapped.end;
  }

  // Note: We could add a final free region from current_addr to a max address,
  // but since the original code stops after the last region, we omit it here
  all_regions
}

fn main() {
  // Get and print the current process ID
  let pid = process::id();
  println!("Process ID: {}", pid);

  // Read mapped regions and generate the full list
  let mapped_regions = read_mapped_regions();
  let all_regions = generate_all_regions(&mapped_regions);

  // Print information for each region
  for region in all_regions {
    match region.region_type {
      RegionType::Mapped {
        ref perms,
        offset,
        ref dev,
        inode,
        ref pathname,
      } => {
        println!(
          "Mapped: {:x}-{:x} perms: {} offset: {:x} dev: {} inode: {} pathname: {}",
          region.start, region.end, perms, offset, dev, inode, pathname
        );
      }
      RegionType::Free => {
        println!("Free: {:x}-{:x}", region.start, region.end);
      }
    }
  }
}
