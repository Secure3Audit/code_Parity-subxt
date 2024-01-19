# READ BEFORE AUDIT!!

This repo is ONLY used for code sharing, please **submit your findings on the contest page**.
- https://www.secure3.io/contest/40e11f9e

**No Informational** issues will be accepted for this contest!
- Read our issue severity definition to understand the difference between **Critical, Medium, Low, and Informational**: https://docs.secure3.io/rewards/severity-standard
- Read our previous audit reports to see some examples of **Critical, Medium, Low, and Informational** issues: https://secure3.io/reports


Please read the below our reward guidelines and submission policy carefully:
- https://docs.secure3.io/rewards/incentive-model
- https://docs.secure3.io/rewards/submission-policy


------

### Audit Time
Starts on: `Jan 19, 2024 - 00:00 UTC`

Ends on: `Feb 19, 2024 - 23:59 UTC`

------

### Project Introduction

Polkadot is a blockspace ecosystem for boundless innovation. Secure, composable, flexible, efficient & cost effective. Powering the movement for a better web.

Below are the project websites and docs:
- [https://polkadot.network/](https://polkadot.network/)
- [https://twitter.com/Polkadot](https://twitter.com/Polkadot)

### Project Dependencies


### Audit Scope
Audit scope **ONLY** includes files listed in the "Code Statistics" section below. 

**Code Statistics**:
```
Original repo commit: `e7a4a8d11d711f9216f3b9f9656ea2828735fe27` 

------------------------------------------------------------------------------------------------------------
File                                                                     blank        comment           code
------------------------------------------------------------------------------------------------------------
./code/subxt/src/blocks/extrinsic_types.rs                                 113            191            726
./code/subxt/src/events/events_type.rs                                     108            151            687
./code/subxt/src/backend/unstable/rpc_methods.rs                            91            266            598
./code/subxt/src/backend/unstable/follow_stream_unpin.rs                   100            135            560
./code/subxt/src/tx/tx_client.rs                                            59            200            549
./code/subxt/src/backend/unstable/mod.rs                                    76             73            506
./code/subxt/src/backend/legacy/rpc_methods.rs                              63            172            442
./code/subxt/src/backend/unstable/follow_stream_driver.rs                   64             74            408
./code/subxt/src/backend/legacy/mod.rs                                      48             48            392
./code/subxt/src/config/signed_extensions.rs                                61             54            381
./code/subxt/src/client/online_client.rs                                    68            154            323
./code/subxt/src/tx/tx_progress.rs                                          40             81            296
./code/subxt/src/backend/unstable/follow_stream.rs                          29             48            243
./code/subxt/src/error/dispatch_error.rs                                    29             68            233
./code/subxt/src/storage/storage_type.rs                                    28             99            226
./code/subxt/src/backend/mod.rs                                             45             89            215
./code/subxt/src/utils/bits.rs                                              30             30            206
./code/subxt/src/client/light_client/builder.rs                             41             97            201
./code/subxt/src/storage/storage_address.rs                                 27             45            198
./code/subxt/src/config/substrate.rs                                        30             54            194
./code/subxt/src/utils/wrapper_opaque.rs                                    32             29            182
./code/subxt/src/backend/rpc/rpc_client.rs                                  28             78            180
./code/subxt/src/utils/account_id.rs                                        24             29            165
./code/subxt/src/error/mod.rs                                               17             72            163
./code/subxt/src/custom_values/custom_values_client.rs                      17             12            150
./code/subxt/src/tx/tx_payload.rs                                           25             34            133
./code/subxt/src/backend/unstable/storage_items.rs                          15             15            130
./code/subxt/src/blocks/block_types.rs                                      19             24            126
./code/subxt/src/blocks/blocks_client.rs                                    14             27            118
./code/subxt/src/client/light_client/mod.rs                                 27             53            114
./code/subxt/src/runtime_api/runtime_payload.rs                             23             48            113
./code/subxt/src/client/light_client/rpc.rs                                 24             31            108
./code/subxt/src/client/offline_client.rs                                   24             33            101
./code/subxt/src/config/default_extrinsic_params.rs                         14             30            100
./code/subxt/src/config/mod.rs                                              26             34             94
./code/subxt/src/utils/unchecked_extrinsic.rs                               22             22             92
./code/subxt/src/runtime_api/runtime_types.rs                               11             15             84
./code/subxt/src/utils/mod.rs                                               16             19             71
./code/subxt/src/utils/era.rs                                                6             32             69
./code/subxt/src/events/events_client.rs                                     9             19             68
./code/subxt/src/constants/constant_address.rs                              15             18             66
./code/subxt/src/storage/storage_client.rs                                   9             21             64
./code/subxt/src/tx/signer.rs                                               14             23             63
./code/subxt/src/constants/constants_client.rs                               7             14             59
./code/subxt/src/backend/rpc/jsonrpsee_impl.rs                               7              3             58
./code/subxt/src/backend/rpc/rpc_client_t.rs                                10             37             56
./code/subxt/src/lib.rs                                                     14            233             56
./code/subxt/src/metadata/metadata_type.rs                                  11              7             56
./code/subxt/src/dynamic.rs                                                 11             20             55
./code/subxt/src/utils/static_type.rs                                       10             13             55
./code/subxt/src/custom_values/custom_value_address.rs                      13             13             54
./code/subxt/src/utils/multi_address.rs                                      7             15             50
./code/subxt/src/macros.rs                                                   8              4             47
./code/subxt/src/config/extrinsic_params.rs                                  9             34             38
./code/subxt/src/metadata/decode_encode_traits.rs                            5              8             38
./code/subxt/src/utils/multi_signature.rs                                    8             12             38
./code/subxt/src/runtime_api/runtime_client.rs                               7             10             36
./code/subxt/src/storage/utils.rs                                            5             11             23
./code/subxt/src/events/mod.rs                                               6             19             19
./code/subxt/src/config/polkadot.rs                                          7              9             17
./code/subxt/src/tx/mod.rs                                                   5             11             17
./code/subxt/src/storage/mod.rs                                              7              8             13
./code/subxt/src/client/mod.rs                                               5              8             12
./code/subxt/src/blocks/mod.rs                                               5              6             11
./code/subxt/src/backend/rpc/mod.rs                                          6             54              8
./code/subxt/src/runtime_api/mod.rs                                          3              4              6
./code/subxt/src/metadata/mod.rs                                             4              5              5
./code/subxt/src/constants/mod.rs                                            3              4              4
./code/subxt/src/custom_values/mod.rs                                        3              4              4
------------------------------------------------------------------------------------------------------------
SUM:                                                                      1767           3413          10973
------------------------------------------------------------------------------------------------------------

```

### Additional Documents
The additional documents provided by the client are under `doc` directory
