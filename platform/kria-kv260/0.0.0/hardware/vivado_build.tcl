# Vivado non-project mode build script for NextMicon
# Usage: vivado -mode batch -source vivado_build.tcl -tclargs <build_dir> <part> <constraints_file>

set build_dir [lindex $argv 0]
set part      [lindex $argv 1]
set xdc_file  [lindex $argv 2]

# Read all SystemVerilog sources from the build directory
foreach f [glob -nocomplain ${build_dir}/*.sv ${build_dir}/platform/*.sv] {
    read_verilog -sv $f
}

# Read constraints
read_xdc $xdc_file

# Synthesis
synth_design -top top -part $part
report_utilization -file ${build_dir}/utilization_synth.rpt

# Implementation
opt_design
place_design
route_design
report_utilization -file ${build_dir}/utilization_impl.rpt
report_timing_summary -file ${build_dir}/timing.rpt

# Generate bitstream
write_bitstream -force ${build_dir}/out.bit
