/*
 * Generated with the FPGA Interface C API Generator 21.3
 * for NI-RIO 21.3 or later.
 */
#ifndef __NiFpga_Test_h__
#define __NiFpga_Test_h__

#ifndef NiFpga_Version
   #define NiFpga_Version 213
#endif

#include "NiFpga.h"

/**
 * The filename of the FPGA bitfile.
 *
 * This is a #define to allow for string literal concatenation. For example:
 *
 *    static const char* const Bitfile = "C:\\" NiFpga_Test_Bitfile;
 */
#define NiFpga_Test_Bitfile "NiFpga_Test.lvbitx"

/**
 * The signature of the FPGA bitfile.
 */
static const char* const NiFpga_Test_Signature = "D08F17F77A45A5692FA2342C6B86E0EE";

#if NiFpga_Cpp
extern "C"
{
#endif

typedef enum
{
   NiFpga_Test_IndicatorBool_FalseBool = 0x18032,
   NiFpga_Test_IndicatorBool_TrueBool = 0x1802E
} NiFpga_Test_IndicatorBool;

typedef enum
{
   NiFpga_Test_IndicatorI8_I8 = 0x18012
} NiFpga_Test_IndicatorI8;

typedef enum
{
   NiFpga_Test_IndicatorU8_U8 = 0x18002
} NiFpga_Test_IndicatorU8;

typedef enum
{
   NiFpga_Test_IndicatorI16_I16 = 0x18016
} NiFpga_Test_IndicatorI16;

typedef enum
{
   NiFpga_Test_IndicatorU16_U16 = 0x18006
} NiFpga_Test_IndicatorU16;

typedef enum
{
   NiFpga_Test_IndicatorI32_I32 = 0x18018
} NiFpga_Test_IndicatorI32;

typedef enum
{
   NiFpga_Test_IndicatorU32_U32 = 0x18008
} NiFpga_Test_IndicatorU32;

typedef enum
{
   NiFpga_Test_IndicatorI64_I64 = 0x1801C
} NiFpga_Test_IndicatorI64;

typedef enum
{
   NiFpga_Test_IndicatorU64_U64 = 0x1800C
} NiFpga_Test_IndicatorU64;

typedef enum
{
   NiFpga_Test_IndicatorSgl_SGL = 0x18020
} NiFpga_Test_IndicatorSgl;

typedef enum
{
   NiFpga_Test_IndicatorArrayBool_BoolArray = 0x18036
} NiFpga_Test_IndicatorArrayBool;

typedef enum
{
   NiFpga_Test_IndicatorArrayBoolSize_BoolArray = 8
} NiFpga_Test_IndicatorArrayBoolSize;

/* Indicator: SignedFXP */
static const NiFpga_FxpTypeInfo NiFpga_Test_IndicatorFxp_SignedFXP_TypeInfo =
{
   1,
   4,
   3
};

/* Use NiFpga_ReadU8() to access SignedFXP */
static const uint32_t NiFpga_Test_IndicatorFxp_SignedFXP_Resource = 0x1802A;


/* Indicator: UnsignedFXP */
static const NiFpga_FxpTypeInfo NiFpga_Test_IndicatorFxp_UnsignedFXP_TypeInfo =
{
   0,
   4,
   3
};

/* Use NiFpga_ReadU8() to access UnsignedFXP */
static const uint32_t NiFpga_Test_IndicatorFxp_UnsignedFXP_Resource = 0x18026;


/* Indicator: TestClusterArray */

/* Use NiFpga_ReadArrayU8() to access TestClusterArray */
static const uint32_t NiFpga_Test_IndicatorClusterArray_TestClusterArray_Resource = 0x1803C;
static const uint32_t NiFpga_Test_IndicatorClusterArray_TestClusterArray_Size = 2;
static const uint32_t NiFpga_Test_IndicatorClusterArray_TestClusterArray_PackedSizeInBytes = 5;

typedef struct NiFpga_Test_IndicatorClusterArray_TestClusterArray_Type{
   NiFpga_Bool b;
   uint16_t u;
}NiFpga_Test_IndicatorClusterArray_TestClusterArray_Type;

void NiFpga_Test_IndicatorClusterArray_TestClusterArray_UnpackArray(
   const uint8_t* const packedData,
   NiFpga_Test_IndicatorClusterArray_TestClusterArray_Type* const destination);

void NiFpga_Test_IndicatorClusterArray_TestClusterArray_PackArray(
   uint8_t* const packedData,
   const NiFpga_Test_IndicatorClusterArray_TestClusterArray_Type* const source);

/* Indicator: TestCluster */
/* Use NiFpga_ReadArrayU8() to access TestCluster */
static const uint32_t NiFpga_Test_IndicatorCluster_TestCluster_Resource = 0x18038;
static const uint32_t NiFpga_Test_IndicatorCluster_TestCluster_PackedSizeInBytes = 3;

typedef struct NiFpga_Test_IndicatorCluster_TestCluster_Type{
   NiFpga_Bool b;
   uint16_t u;
}NiFpga_Test_IndicatorCluster_TestCluster_Type;


void NiFpga_Test_IndicatorCluster_TestCluster_UnpackCluster(
   const uint8_t* const packedData,
   NiFpga_Test_IndicatorCluster_TestCluster_Type* const destination);

void NiFpga_Test_IndicatorCluster_TestCluster_PackCluster(
   uint8_t* const packedData,
   const NiFpga_Test_IndicatorCluster_TestCluster_Type* const source);


#if NiFpga_Cpp
}
#endif

#endif
