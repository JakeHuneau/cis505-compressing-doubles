# Introduction

* Time series Database (TSDB)
* Always available for writes

# Requirements
* 2B unique time series identified with string key
* 700M data points added per minute
* Data stored for 26 hours
* > 40k queries per second at peak
* reads succeed in < 1ms
* Support time series with 15 second granularity (4 points per minute per time series)
* 2 in-memory, no co-located replicas
* always serve reads even when a single server crashes
* ability to quickly scan over all in-memory data
* support at least 2x growth per year

# Architecture
* data stored with 3-tuple of (string key, 64 bit time stamp integer, double precision floating point value)
* Can compress a stream
* Full resolution is kept
* Compresses data stream into blocks, partitioned by time
* Data is (time, data)

## Time stamp compression
* delta of deltas
* For example, if the deltas between points are 60, 60, 59, 61 then the delta of deltas is 0, -1, 2
* Then encode with variable length encoding with the following algorithm
    1. block header stores the starting time stamp, t_{-1}, aligned to a 2 hour window; the first time stamp `t_0`, in the block is stored as a delta from `t_{-1}` in 14 bits.
    2. Then the subsequent time stamps `t_n`:
        1. Calculate delta of deltas: D = (t_n - t_{n-1}) - (t_{n-1} - t_{n-2})
        2. If D is zero, then store a single `0` bit
        3. If D is between [-63, 64] store `10` folowed by the value (7 bits)
        4. IF D is between [-255, 256] store `110` followed by the value (9 bits)
        5. If D is between [-2047, 2048] store `1110` followed by the value (12 bits)
        6. Otherwise, store `1111` followed by D using 32 bits

## Compressing Values
* double floating point type
* Use XOR of current and previous value. Then do the following with the result for a variable-length:
    1. First value is stored without compression
    2. If XOR with previous is zero (same value), store single `0`
    3. When XOR is non-zero, calculate number of leading and trailing zeros in the XOR, store `1` followed by either
        1. (Control bit `0`) If the block of meaningful bits falls within the block of previous meaningful bits (there are at least as many leading zeros and as many trailing
zeros as with the previous value), use that information for the block position and just store the meaningful XOR value
        2. (Control bit `1`) Store the length of the number of leading zeros in the next 5 bits, then store the length of the meaningful XORed value in the next 6 bits.
Finally, store the meaningful bits of the XORed value.

## Data Structures
* Timeseries Map (TSmap)
* Vector of pointers to time series and a case-insensitive, case-preserving map from time series name to the same
