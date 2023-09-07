/*
 * Generated with the FPGA Interface C API Generator 21.3
 * for NI-RIO 21.3 or later.
 */
#include "NiFpga_Test.h"

void NiFpga_Test_IndicatorClusterArray_TestClusterArray_UnpackArray(
   const uint8_t* const packedData,
   NiFpga_Test_IndicatorClusterArray_TestClusterArray_Type* const destination)
{
   destination[0].b = 0;
   destination[0].b |= ((packedData[0] >> 7) & 0x1);
   destination[0].u = 0;
   destination[0].u |= (packedData[0] & 0x7F) << 9;
   destination[0].u |= (packedData[1] & 0xFF) << 1;
   destination[0].u |= ((packedData[2] >> 7) & 0x1);
   destination[1].b = 0;
   destination[1].b |= ((packedData[2] >> 6) & 0x1);
   destination[1].u = 0;
   destination[1].u |= (packedData[2] & 0x3F) << 10;
   destination[1].u |= (packedData[3] & 0xFF) << 2;
   destination[1].u |= ((packedData[4] >> 6) & 0x3);
}

void NiFpga_Test_IndicatorClusterArray_TestClusterArray_PackArray(
   uint8_t* const packedData,
   const NiFpga_Test_IndicatorClusterArray_TestClusterArray_Type* const source)
{
   packedData[0] = (uint8_t)((source[0].b & 0x1) << 7);
   packedData[0] |= (uint8_t)((source[0].u >> 9) & 0x7F);
   packedData[1] = (uint8_t)((source[0].u >> 1) & 0xFF);
   packedData[2] = (uint8_t)((source[0].u & 0x1) << 7);
   packedData[2] |= (uint8_t)((source[1].b & 0x1) << 6);
   packedData[2] |= (uint8_t)((source[1].u >> 10) & 0x3F);
   packedData[3] = (uint8_t)((source[1].u >> 2) & 0xFF);
   packedData[4] = (uint8_t)((source[1].u & 0x3) << 6);
}

void NiFpga_Test_IndicatorCluster_TestCluster_UnpackCluster(
   const uint8_t* const packedData,
   NiFpga_Test_IndicatorCluster_TestCluster_Type* const destination)
{
   (*destination).b = 0;
   (*destination).b |= (packedData[0] & 0x1);
   (*destination).u = 0;
   (*destination).u |= (packedData[1] & 0xFF) << 8;
   (*destination).u |= (packedData[2] & 0xFF);
}

void NiFpga_Test_IndicatorCluster_TestCluster_PackCluster(
   uint8_t* const packedData,
   const NiFpga_Test_IndicatorCluster_TestCluster_Type* const source)
{
   packedData[0] |= (uint8_t)((*source).b & 0x1);
   packedData[1] = (uint8_t)(((*source).u >> 8) & 0xFF);
   packedData[2] = (uint8_t)((*source).u & 0xFF);
}
