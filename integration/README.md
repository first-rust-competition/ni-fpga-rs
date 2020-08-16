# integration
This package builds a binary that runs on the roboRIO for integration testing.
It includes its own FPGA bitfile that supplies some registers containing fixture data that it attempts to read.

Run `/etc/init.d/FRCNetComm stop` before running on the roboRIO, otherwise you won't be able to download the fixture bitfile to the FPGA.
