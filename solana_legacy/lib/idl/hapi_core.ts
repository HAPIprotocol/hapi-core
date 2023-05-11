export type HapiCore = {
  "version": "0.2.0",
  "name": "hapi_core",
  "instructions": [
    {
      "name": "initializeCommunity",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "community",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityId",
          "type": "u64"
        },
        {
          "name": "bump",
          "type": "u8"
        },
        {
          "name": "stakeUnlockEpochs",
          "type": "u64"
        },
        {
          "name": "confirmationThreshold",
          "type": "u8"
        },
        {
          "name": "validatorStake",
          "type": "u64"
        },
        {
          "name": "tracerStake",
          "type": "u64"
        },
        {
          "name": "fullStake",
          "type": "u64"
        },
        {
          "name": "authorityStake",
          "type": "u64"
        },
        {
          "name": "appraiserStake",
          "type": "u64"
        }
      ]
    },
    {
      "name": "updateCommunity",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "stakeUnlockEpochs",
          "type": "u64"
        },
        {
          "name": "confirmationThreshold",
          "type": "u8"
        },
        {
          "name": "validatorStake",
          "type": "u64"
        },
        {
          "name": "tracerStake",
          "type": "u64"
        },
        {
          "name": "fullStake",
          "type": "u64"
        },
        {
          "name": "authorityStake",
          "type": "u64"
        },
        {
          "name": "appraiserStake",
          "type": "u64"
        }
      ]
    },
    {
      "name": "migrateCommunity",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "oldCommunity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "oldTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenSigner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityId",
          "type": "u64"
        },
        {
          "name": "bump",
          "type": "u8"
        },
        {
          "name": "tokenSignerBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "setCommunityAuthority",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "newAuthority",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createNetwork",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "name",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "schema",
          "type": {
            "defined": "NetworkSchema"
          }
        },
        {
          "name": "addressTracerReward",
          "type": "u64"
        },
        {
          "name": "addressConfirmationReward",
          "type": "u64"
        },
        {
          "name": "assetTracerReward",
          "type": "u64"
        },
        {
          "name": "assetConfirmationReward",
          "type": "u64"
        },
        {
          "name": "networkBump",
          "type": "u8"
        },
        {
          "name": "reportPrice",
          "type": "u64"
        }
      ]
    },
    {
      "name": "updateNetwork",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "addressTracerReward",
          "type": "u64"
        },
        {
          "name": "addressConfirmationReward",
          "type": "u64"
        },
        {
          "name": "assetTracerReward",
          "type": "u64"
        },
        {
          "name": "assetConfirmationReward",
          "type": "u64"
        }
      ]
    },
    {
      "name": "migrateNetwork",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "oldNetwork",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardSigner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "name",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "bump",
          "type": "u8"
        },
        {
          "name": "rewardSignerBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createReporter",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pubkey",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "role",
          "type": {
            "defined": "ReporterRole"
          }
        },
        {
          "name": "name",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "updateReporter",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "role",
          "type": {
            "defined": "ReporterRole"
          }
        },
        {
          "name": "name",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        }
      ]
    },
    {
      "name": "migrateReporter",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oldReporter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pubkey",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "migrateReporterReward",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oldReporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createCase",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "caseId",
          "type": "u64"
        },
        {
          "name": "name",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "updateCase",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "name",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "status",
          "type": {
            "defined": "CaseStatus"
          }
        }
      ]
    },
    {
      "name": "migrateCase",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oldCase",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "caseId",
          "type": "u64"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createAddress",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "address",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporterPaymentTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "addr",
          "type": {
            "array": [
              "u8",
              64
            ]
          }
        },
        {
          "name": "category",
          "type": {
            "defined": "Category"
          }
        },
        {
          "name": "risk",
          "type": "u8"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "confirmAddress",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "addressReporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "address",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updateAddress",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "address",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporterPaymentTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "category",
          "type": {
            "defined": "Category"
          }
        },
        {
          "name": "risk",
          "type": "u8"
        }
      ]
    },
    {
      "name": "migrateAddress",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oldAddress",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "address",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "addr",
          "type": {
            "array": [
              "u8",
              64
            ]
          }
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "changeAddressCase",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "newCase",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "address",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createAsset",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "asset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporterPaymentTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "mint",
          "type": {
            "array": [
              "u8",
              64
            ]
          }
        },
        {
          "name": "assetId",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "category",
          "type": {
            "defined": "Category"
          }
        },
        {
          "name": "risk",
          "type": "u8"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "confirmAsset",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "assetReporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "asset",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updateAsset",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "asset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporterPaymentTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "category",
          "type": {
            "defined": "Category"
          }
        },
        {
          "name": "risk",
          "type": "u8"
        }
      ]
    },
    {
      "name": "migrateAsset",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oldAsset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "asset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "mint",
          "type": {
            "array": [
              "u8",
              64
            ]
          }
        },
        {
          "name": "assetId",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "initializeReporterReward",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "activateReporter",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stakeMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporterTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "communityTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "deactivateReporter",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "releaseReporter",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stakeMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporterTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "communityTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "claimReporterReward",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporterTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "freezeReporter",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "unfreezeReporter",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updateReplicationPrice",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "price",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "address",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "Account version"
            ],
            "type": "u16"
          },
          {
            "name": "community",
            "docs": [
              "Community account, which this address belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "network",
            "docs": [
              "Network account, which this address belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "address",
            "docs": [
              "Actual address public key"
            ],
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "caseId",
            "docs": [
              "ID of the associated case"
            ],
            "type": "u64"
          },
          {
            "name": "reporter",
            "docs": [
              "Reporter account public key"
            ],
            "type": "publicKey"
          },
          {
            "name": "category",
            "docs": [
              "Category of illicit activity identified with this address"
            ],
            "type": {
              "defined": "Category"
            }
          },
          {
            "name": "risk",
            "docs": [
              "Address risk score 0..10 (0 is safe, 10 is maximum risk)"
            ],
            "type": "u8"
          },
          {
            "name": "confirmations",
            "docs": [
              "Confirmation count for this address"
            ],
            "type": "u8"
          },
          {
            "name": "replicationBounty",
            "docs": [
              "Accumulated payment amount for report"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "asset",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "Account version"
            ],
            "type": "u16"
          },
          {
            "name": "community",
            "docs": [
              "Community account, which this address belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "network",
            "docs": [
              "Network account, which this address belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "mint",
            "docs": [
              "Asset mint account"
            ],
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "assetId",
            "docs": [
              "Asset ID"
            ],
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "caseId",
            "docs": [
              "ID of the associated case"
            ],
            "type": "u64"
          },
          {
            "name": "reporter",
            "docs": [
              "Reporter account public key"
            ],
            "type": "publicKey"
          },
          {
            "name": "category",
            "docs": [
              "Category of illicit activity identified with this address"
            ],
            "type": {
              "defined": "Category"
            }
          },
          {
            "name": "risk",
            "docs": [
              "Address risk score 0..10 (0 is safe, 10 is maximum risk)"
            ],
            "type": "u8"
          },
          {
            "name": "confirmations",
            "docs": [
              "Confirmation count for this address"
            ],
            "type": "u8"
          },
          {
            "name": "replicationBounty",
            "docs": [
              "Accumulated payment amount for report"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "case",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "Account version"
            ],
            "type": "u16"
          },
          {
            "name": "community",
            "docs": [
              "Community account, which this case belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "id",
            "docs": [
              "Sequantial case ID"
            ],
            "type": "u64"
          },
          {
            "name": "reporter",
            "docs": [
              "Case reporter's account"
            ],
            "type": "publicKey"
          },
          {
            "name": "status",
            "docs": [
              "Case status"
            ],
            "type": {
              "defined": "CaseStatus"
            }
          },
          {
            "name": "name",
            "docs": [
              "Short case description"
            ],
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "community",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "Account version"
            ],
            "type": "u16"
          },
          {
            "name": "authority",
            "docs": [
              "Community authority wallet"
            ],
            "type": "publicKey"
          },
          {
            "name": "id",
            "docs": [
              "Community ID"
            ],
            "type": "u64"
          },
          {
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "cases",
            "docs": [
              "Community case counter"
            ],
            "type": "u64"
          },
          {
            "name": "confirmationThreshold",
            "docs": [
              "Number of confirmations needed for address to be considered confirmed"
            ],
            "type": "u8"
          },
          {
            "name": "stakeUnlockEpochs",
            "docs": [
              "Number of epochs reporter must wait to retrieve their stake"
            ],
            "type": "u64"
          },
          {
            "name": "stakeMint",
            "docs": [
              "Stake token mint account"
            ],
            "type": "publicKey"
          },
          {
            "name": "validatorStake",
            "docs": [
              "Amount of stake required from a reporter of validator type"
            ],
            "type": "u64"
          },
          {
            "name": "tracerStake",
            "docs": [
              "Amount of stake required from a reporter of tracer type"
            ],
            "type": "u64"
          },
          {
            "name": "fullStake",
            "docs": [
              "Amount of stake required from a reporter of full type"
            ],
            "type": "u64"
          },
          {
            "name": "authorityStake",
            "docs": [
              "Amount of stake required from a reporter of authority type"
            ],
            "type": "u64"
          },
          {
            "name": "appraiserStake",
            "docs": [
              "Amount of stake required from a reporter of appraiser type"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "addressV0",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "community",
            "type": "publicKey"
          },
          {
            "name": "network",
            "type": "publicKey"
          },
          {
            "name": "address",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "caseId",
            "type": "u64"
          },
          {
            "name": "reporter",
            "type": "publicKey"
          },
          {
            "name": "category",
            "type": {
              "defined": "Category"
            }
          },
          {
            "name": "risk",
            "type": "u8"
          },
          {
            "name": "confirmations",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "assetV0",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "community",
            "type": "publicKey"
          },
          {
            "name": "network",
            "type": "publicKey"
          },
          {
            "name": "mint",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "assetId",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "caseId",
            "type": "u64"
          },
          {
            "name": "reporter",
            "type": "publicKey"
          },
          {
            "name": "category",
            "type": {
              "defined": "Category"
            }
          },
          {
            "name": "risk",
            "type": "u8"
          },
          {
            "name": "confirmations",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "caseV0",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "community",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "id",
            "type": "u64"
          },
          {
            "name": "reporter",
            "type": "publicKey"
          },
          {
            "name": "status",
            "type": {
              "defined": "CaseStatus"
            }
          },
          {
            "name": "name",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "communityV0",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "cases",
            "type": "u64"
          },
          {
            "name": "confirmationThreshold",
            "type": "u8"
          },
          {
            "name": "stakeUnlockEpochs",
            "type": "u64"
          },
          {
            "name": "stakeMint",
            "type": "publicKey"
          },
          {
            "name": "tokenSigner",
            "type": "publicKey"
          },
          {
            "name": "tokenSignerBump",
            "type": "u8"
          },
          {
            "name": "tokenAccount",
            "type": "publicKey"
          },
          {
            "name": "validatorStake",
            "type": "u64"
          },
          {
            "name": "tracerStake",
            "type": "u64"
          },
          {
            "name": "fullStake",
            "type": "u64"
          },
          {
            "name": "authorityStake",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "networkV0",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "community",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "name",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "schema",
            "type": {
              "defined": "NetworkSchema"
            }
          },
          {
            "name": "rewardMint",
            "type": "publicKey"
          },
          {
            "name": "rewardSigner",
            "type": "publicKey"
          },
          {
            "name": "rewardSignerBump",
            "type": "u8"
          },
          {
            "name": "addressTracerReward",
            "type": "u64"
          },
          {
            "name": "addressConfirmationReward",
            "type": "u64"
          },
          {
            "name": "assetTracerReward",
            "type": "u64"
          },
          {
            "name": "assetConfirmationReward",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "reporterRewardV0",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reporter",
            "type": "publicKey"
          },
          {
            "name": "network",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "addressTracerCounter",
            "type": "u64"
          },
          {
            "name": "addressConfirmationCounter",
            "type": "u64"
          },
          {
            "name": "assetTracerCounter",
            "type": "u64"
          },
          {
            "name": "assetConfirmationCounter",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "reporterV0",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "community",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "isFrozen",
            "type": "bool"
          },
          {
            "name": "status",
            "type": {
              "defined": "ReporterStatus"
            }
          },
          {
            "name": "role",
            "type": {
              "defined": "ReporterRole"
            }
          },
          {
            "name": "pubkey",
            "type": "publicKey"
          },
          {
            "name": "name",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "stake",
            "type": "u64"
          },
          {
            "name": "unlockEpoch",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "network",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "Account version"
            ],
            "type": "u16"
          },
          {
            "name": "community",
            "docs": [
              "Community account, which this network belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "name",
            "docs": [
              "Network name (i.e. ethereum, solana, near)"
            ],
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "schema",
            "type": {
              "defined": "NetworkSchema"
            }
          },
          {
            "name": "rewardMint",
            "docs": [
              "Reward token mint account"
            ],
            "type": "publicKey"
          },
          {
            "name": "addressTracerReward",
            "docs": [
              "Reward amount for tracers that report addresses to this network"
            ],
            "type": "u64"
          },
          {
            "name": "addressConfirmationReward",
            "docs": [
              "Reward amount for tracers and validators that confirm addresses on this network"
            ],
            "type": "u64"
          },
          {
            "name": "assetTracerReward",
            "docs": [
              "Reward amount for tracers that report assets to this network"
            ],
            "type": "u64"
          },
          {
            "name": "assetConfirmationReward",
            "docs": [
              "Reward amount for tracers and validators that confirm assets on this network"
            ],
            "type": "u64"
          },
          {
            "name": "replicationPrice",
            "docs": [
              "Replication price amount"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "reporter",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "Account version"
            ],
            "type": "u16"
          },
          {
            "name": "community",
            "docs": [
              "Community account, which this reporter belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "isFrozen",
            "docs": [
              "If this is true, reporter can't interact with the contract"
            ],
            "type": "bool"
          },
          {
            "name": "status",
            "docs": [
              "Reporter account status"
            ],
            "type": {
              "defined": "ReporterStatus"
            }
          },
          {
            "name": "role",
            "docs": [
              "Reporter's type"
            ],
            "type": {
              "defined": "ReporterRole"
            }
          },
          {
            "name": "pubkey",
            "docs": [
              "Reporter's wallet account"
            ],
            "type": "publicKey"
          },
          {
            "name": "name",
            "docs": [
              "Short reporter description"
            ],
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "stake",
            "docs": [
              "Current deposited stake"
            ],
            "type": "u64"
          },
          {
            "name": "unlockEpoch",
            "docs": [
              "Reporter can unstake at this epoch (0 if unstaking hasn't been requested)"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "reporterReward",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "Account version"
            ],
            "type": "u16"
          },
          {
            "name": "reporter",
            "docs": [
              "Reporter account to keep reward counter for"
            ],
            "type": "publicKey"
          },
          {
            "name": "network",
            "docs": [
              "Network that has the reward associated with"
            ],
            "type": "publicKey"
          },
          {
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "addressTracerCounter",
            "docs": [
              "Number of unclaimed address report rewards"
            ],
            "type": "u64"
          },
          {
            "name": "addressConfirmationCounter",
            "docs": [
              "Number of unclaimed address confirmation rewards"
            ],
            "type": "u64"
          },
          {
            "name": "assetTracerCounter",
            "docs": [
              "Number of unclaimed asset report rewards"
            ],
            "type": "u64"
          },
          {
            "name": "assetConfirmationCounter",
            "docs": [
              "Number of unclaimed asset confirmation rewards"
            ],
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "HapiEnvironment",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Devnet"
          },
          {
            "name": "Mainnet"
          }
        ]
      }
    },
    {
      "name": "Category",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "WalletService"
          },
          {
            "name": "MerchantService"
          },
          {
            "name": "MiningPool"
          },
          {
            "name": "Exchange"
          },
          {
            "name": "DeFi"
          },
          {
            "name": "OTCBroker"
          },
          {
            "name": "ATM"
          },
          {
            "name": "Gambling"
          },
          {
            "name": "IllicitOrganization"
          },
          {
            "name": "Mixer"
          },
          {
            "name": "DarknetService"
          },
          {
            "name": "Scam"
          },
          {
            "name": "Ransomware"
          },
          {
            "name": "Theft"
          },
          {
            "name": "Counterfeit"
          },
          {
            "name": "TerroristFinancing"
          },
          {
            "name": "Sanctions"
          },
          {
            "name": "ChildAbuse"
          },
          {
            "name": "Hacker"
          },
          {
            "name": "HighRiskJurisdiction"
          }
        ]
      }
    },
    {
      "name": "CaseStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Closed"
          },
          {
            "name": "Open"
          }
        ]
      }
    },
    {
      "name": "NetworkSchema",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Plain"
          },
          {
            "name": "Solana"
          },
          {
            "name": "Ethereum"
          },
          {
            "name": "Bitcoin"
          },
          {
            "name": "Near"
          }
        ]
      }
    },
    {
      "name": "ReporterStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Inactive"
          },
          {
            "name": "Active"
          },
          {
            "name": "Unstaking"
          }
        ]
      }
    },
    {
      "name": "ReporterRole",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Validator"
          },
          {
            "name": "Tracer"
          },
          {
            "name": "Publisher"
          },
          {
            "name": "Authority"
          },
          {
            "name": "Appraiser"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "UnexpectedAccount",
      "msg": "Unexpected account has been used"
    },
    {
      "code": 6001,
      "name": "Unauthorized",
      "msg": "Account is not authorized to perform this action"
    },
    {
      "code": 6002,
      "name": "NonSequentialCaseId",
      "msg": "Non-sequential case ID"
    },
    {
      "code": 6003,
      "name": "ReleaseEpochInFuture",
      "msg": "Release epoch is in future"
    },
    {
      "code": 6004,
      "name": "InvalidMint",
      "msg": "Invalid mint account"
    },
    {
      "code": 6005,
      "name": "InvalidReporter",
      "msg": "Invalid reporter account"
    },
    {
      "code": 6006,
      "name": "InactiveReporter",
      "msg": "Reporter account is not active"
    },
    {
      "code": 6007,
      "name": "InvalidToken",
      "msg": "Invalid token account"
    },
    {
      "code": 6008,
      "name": "CaseClosed",
      "msg": "Case closed"
    },
    {
      "code": 6009,
      "name": "InvalidReporterStatus",
      "msg": "Invalid reporter status"
    },
    {
      "code": 6010,
      "name": "AuthorityMismatch",
      "msg": "Authority mismatched"
    },
    {
      "code": 6011,
      "name": "CommunityMismatch",
      "msg": "Community mismatched"
    },
    {
      "code": 6012,
      "name": "FrozenReporter",
      "msg": "This reporter is frozen"
    },
    {
      "code": 6013,
      "name": "RiskOutOfRange",
      "msg": "Risk score must be in 0..10 range"
    },
    {
      "code": 6014,
      "name": "NetworkMismatch",
      "msg": "Network mismatched"
    },
    {
      "code": 6015,
      "name": "CaseMismatch",
      "msg": "Case mismatched"
    },
    {
      "code": 6016,
      "name": "SameCase",
      "msg": "Same address case"
    },
    {
      "code": 6017,
      "name": "NoReward",
      "msg": "There is no reward to claim"
    },
    {
      "code": 6018,
      "name": "IllegalOwner",
      "msg": "Account has illegal owner"
    },
    {
      "code": 6019,
      "name": "HighAccountRisk",
      "msg": "User account has high risk"
    },
    {
      "code": 6020,
      "name": "UnexpectedLength",
      "msg": "Unexpected account length"
    },
    {
      "code": 6021,
      "name": "InvalidAccountVersion",
      "msg": "Invalid account version"
    }
  ]
};

export const IDL: HapiCore = {
  "version": "0.2.0",
  "name": "hapi_core",
  "instructions": [
    {
      "name": "initializeCommunity",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "community",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityId",
          "type": "u64"
        },
        {
          "name": "bump",
          "type": "u8"
        },
        {
          "name": "stakeUnlockEpochs",
          "type": "u64"
        },
        {
          "name": "confirmationThreshold",
          "type": "u8"
        },
        {
          "name": "validatorStake",
          "type": "u64"
        },
        {
          "name": "tracerStake",
          "type": "u64"
        },
        {
          "name": "fullStake",
          "type": "u64"
        },
        {
          "name": "authorityStake",
          "type": "u64"
        },
        {
          "name": "appraiserStake",
          "type": "u64"
        }
      ]
    },
    {
      "name": "updateCommunity",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "stakeUnlockEpochs",
          "type": "u64"
        },
        {
          "name": "confirmationThreshold",
          "type": "u8"
        },
        {
          "name": "validatorStake",
          "type": "u64"
        },
        {
          "name": "tracerStake",
          "type": "u64"
        },
        {
          "name": "fullStake",
          "type": "u64"
        },
        {
          "name": "authorityStake",
          "type": "u64"
        },
        {
          "name": "appraiserStake",
          "type": "u64"
        }
      ]
    },
    {
      "name": "migrateCommunity",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "oldCommunity",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "oldTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenSigner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "communityId",
          "type": "u64"
        },
        {
          "name": "bump",
          "type": "u8"
        },
        {
          "name": "tokenSignerBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "setCommunityAuthority",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "newAuthority",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createNetwork",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "name",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "schema",
          "type": {
            "defined": "NetworkSchema"
          }
        },
        {
          "name": "addressTracerReward",
          "type": "u64"
        },
        {
          "name": "addressConfirmationReward",
          "type": "u64"
        },
        {
          "name": "assetTracerReward",
          "type": "u64"
        },
        {
          "name": "assetConfirmationReward",
          "type": "u64"
        },
        {
          "name": "networkBump",
          "type": "u8"
        },
        {
          "name": "reportPrice",
          "type": "u64"
        }
      ]
    },
    {
      "name": "updateNetwork",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "addressTracerReward",
          "type": "u64"
        },
        {
          "name": "addressConfirmationReward",
          "type": "u64"
        },
        {
          "name": "assetTracerReward",
          "type": "u64"
        },
        {
          "name": "assetConfirmationReward",
          "type": "u64"
        }
      ]
    },
    {
      "name": "migrateNetwork",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "oldNetwork",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardSigner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "name",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "bump",
          "type": "u8"
        },
        {
          "name": "rewardSignerBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createReporter",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pubkey",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "role",
          "type": {
            "defined": "ReporterRole"
          }
        },
        {
          "name": "name",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "updateReporter",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "role",
          "type": {
            "defined": "ReporterRole"
          }
        },
        {
          "name": "name",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        }
      ]
    },
    {
      "name": "migrateReporter",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oldReporter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "pubkey",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "migrateReporterReward",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oldReporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createCase",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "caseId",
          "type": "u64"
        },
        {
          "name": "name",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "updateCase",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "name",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "status",
          "type": {
            "defined": "CaseStatus"
          }
        }
      ]
    },
    {
      "name": "migrateCase",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oldCase",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "caseId",
          "type": "u64"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createAddress",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "address",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporterPaymentTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "addr",
          "type": {
            "array": [
              "u8",
              64
            ]
          }
        },
        {
          "name": "category",
          "type": {
            "defined": "Category"
          }
        },
        {
          "name": "risk",
          "type": "u8"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "confirmAddress",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "addressReporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "address",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updateAddress",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "address",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporterPaymentTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "category",
          "type": {
            "defined": "Category"
          }
        },
        {
          "name": "risk",
          "type": "u8"
        }
      ]
    },
    {
      "name": "migrateAddress",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oldAddress",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "address",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "addr",
          "type": {
            "array": [
              "u8",
              64
            ]
          }
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "changeAddressCase",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "newCase",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "address",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createAsset",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "asset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporterPaymentTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "mint",
          "type": {
            "array": [
              "u8",
              64
            ]
          }
        },
        {
          "name": "assetId",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "category",
          "type": {
            "defined": "Category"
          }
        },
        {
          "name": "risk",
          "type": "u8"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "confirmAsset",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "assetReporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "asset",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updateAsset",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "case",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "asset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporterPaymentTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "category",
          "type": {
            "defined": "Category"
          }
        },
        {
          "name": "risk",
          "type": "u8"
        }
      ]
    },
    {
      "name": "migrateAsset",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oldAsset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "asset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "mint",
          "type": {
            "array": [
              "u8",
              64
            ]
          }
        },
        {
          "name": "assetId",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "initializeReporterReward",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "activateReporter",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stakeMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporterTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "communityTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "deactivateReporter",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "releaseReporter",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stakeMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporterTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "communityTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "claimReporterReward",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporterReward",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporterTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "freezeReporter",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "unfreezeReporter",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updateReplicationPrice",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "community",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "network",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "reporter",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "price",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "address",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "Account version"
            ],
            "type": "u16"
          },
          {
            "name": "community",
            "docs": [
              "Community account, which this address belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "network",
            "docs": [
              "Network account, which this address belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "address",
            "docs": [
              "Actual address public key"
            ],
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "caseId",
            "docs": [
              "ID of the associated case"
            ],
            "type": "u64"
          },
          {
            "name": "reporter",
            "docs": [
              "Reporter account public key"
            ],
            "type": "publicKey"
          },
          {
            "name": "category",
            "docs": [
              "Category of illicit activity identified with this address"
            ],
            "type": {
              "defined": "Category"
            }
          },
          {
            "name": "risk",
            "docs": [
              "Address risk score 0..10 (0 is safe, 10 is maximum risk)"
            ],
            "type": "u8"
          },
          {
            "name": "confirmations",
            "docs": [
              "Confirmation count for this address"
            ],
            "type": "u8"
          },
          {
            "name": "replicationBounty",
            "docs": [
              "Accumulated payment amount for report"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "asset",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "Account version"
            ],
            "type": "u16"
          },
          {
            "name": "community",
            "docs": [
              "Community account, which this address belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "network",
            "docs": [
              "Network account, which this address belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "mint",
            "docs": [
              "Asset mint account"
            ],
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "assetId",
            "docs": [
              "Asset ID"
            ],
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "caseId",
            "docs": [
              "ID of the associated case"
            ],
            "type": "u64"
          },
          {
            "name": "reporter",
            "docs": [
              "Reporter account public key"
            ],
            "type": "publicKey"
          },
          {
            "name": "category",
            "docs": [
              "Category of illicit activity identified with this address"
            ],
            "type": {
              "defined": "Category"
            }
          },
          {
            "name": "risk",
            "docs": [
              "Address risk score 0..10 (0 is safe, 10 is maximum risk)"
            ],
            "type": "u8"
          },
          {
            "name": "confirmations",
            "docs": [
              "Confirmation count for this address"
            ],
            "type": "u8"
          },
          {
            "name": "replicationBounty",
            "docs": [
              "Accumulated payment amount for report"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "case",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "Account version"
            ],
            "type": "u16"
          },
          {
            "name": "community",
            "docs": [
              "Community account, which this case belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "id",
            "docs": [
              "Sequantial case ID"
            ],
            "type": "u64"
          },
          {
            "name": "reporter",
            "docs": [
              "Case reporter's account"
            ],
            "type": "publicKey"
          },
          {
            "name": "status",
            "docs": [
              "Case status"
            ],
            "type": {
              "defined": "CaseStatus"
            }
          },
          {
            "name": "name",
            "docs": [
              "Short case description"
            ],
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "community",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "Account version"
            ],
            "type": "u16"
          },
          {
            "name": "authority",
            "docs": [
              "Community authority wallet"
            ],
            "type": "publicKey"
          },
          {
            "name": "id",
            "docs": [
              "Community ID"
            ],
            "type": "u64"
          },
          {
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "cases",
            "docs": [
              "Community case counter"
            ],
            "type": "u64"
          },
          {
            "name": "confirmationThreshold",
            "docs": [
              "Number of confirmations needed for address to be considered confirmed"
            ],
            "type": "u8"
          },
          {
            "name": "stakeUnlockEpochs",
            "docs": [
              "Number of epochs reporter must wait to retrieve their stake"
            ],
            "type": "u64"
          },
          {
            "name": "stakeMint",
            "docs": [
              "Stake token mint account"
            ],
            "type": "publicKey"
          },
          {
            "name": "validatorStake",
            "docs": [
              "Amount of stake required from a reporter of validator type"
            ],
            "type": "u64"
          },
          {
            "name": "tracerStake",
            "docs": [
              "Amount of stake required from a reporter of tracer type"
            ],
            "type": "u64"
          },
          {
            "name": "fullStake",
            "docs": [
              "Amount of stake required from a reporter of full type"
            ],
            "type": "u64"
          },
          {
            "name": "authorityStake",
            "docs": [
              "Amount of stake required from a reporter of authority type"
            ],
            "type": "u64"
          },
          {
            "name": "appraiserStake",
            "docs": [
              "Amount of stake required from a reporter of appraiser type"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "addressV0",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "community",
            "type": "publicKey"
          },
          {
            "name": "network",
            "type": "publicKey"
          },
          {
            "name": "address",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "caseId",
            "type": "u64"
          },
          {
            "name": "reporter",
            "type": "publicKey"
          },
          {
            "name": "category",
            "type": {
              "defined": "Category"
            }
          },
          {
            "name": "risk",
            "type": "u8"
          },
          {
            "name": "confirmations",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "assetV0",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "community",
            "type": "publicKey"
          },
          {
            "name": "network",
            "type": "publicKey"
          },
          {
            "name": "mint",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "assetId",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "caseId",
            "type": "u64"
          },
          {
            "name": "reporter",
            "type": "publicKey"
          },
          {
            "name": "category",
            "type": {
              "defined": "Category"
            }
          },
          {
            "name": "risk",
            "type": "u8"
          },
          {
            "name": "confirmations",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "caseV0",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "community",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "id",
            "type": "u64"
          },
          {
            "name": "reporter",
            "type": "publicKey"
          },
          {
            "name": "status",
            "type": {
              "defined": "CaseStatus"
            }
          },
          {
            "name": "name",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "communityV0",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "cases",
            "type": "u64"
          },
          {
            "name": "confirmationThreshold",
            "type": "u8"
          },
          {
            "name": "stakeUnlockEpochs",
            "type": "u64"
          },
          {
            "name": "stakeMint",
            "type": "publicKey"
          },
          {
            "name": "tokenSigner",
            "type": "publicKey"
          },
          {
            "name": "tokenSignerBump",
            "type": "u8"
          },
          {
            "name": "tokenAccount",
            "type": "publicKey"
          },
          {
            "name": "validatorStake",
            "type": "u64"
          },
          {
            "name": "tracerStake",
            "type": "u64"
          },
          {
            "name": "fullStake",
            "type": "u64"
          },
          {
            "name": "authorityStake",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "networkV0",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "community",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "name",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "schema",
            "type": {
              "defined": "NetworkSchema"
            }
          },
          {
            "name": "rewardMint",
            "type": "publicKey"
          },
          {
            "name": "rewardSigner",
            "type": "publicKey"
          },
          {
            "name": "rewardSignerBump",
            "type": "u8"
          },
          {
            "name": "addressTracerReward",
            "type": "u64"
          },
          {
            "name": "addressConfirmationReward",
            "type": "u64"
          },
          {
            "name": "assetTracerReward",
            "type": "u64"
          },
          {
            "name": "assetConfirmationReward",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "reporterRewardV0",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reporter",
            "type": "publicKey"
          },
          {
            "name": "network",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "addressTracerCounter",
            "type": "u64"
          },
          {
            "name": "addressConfirmationCounter",
            "type": "u64"
          },
          {
            "name": "assetTracerCounter",
            "type": "u64"
          },
          {
            "name": "assetConfirmationCounter",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "reporterV0",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "community",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "isFrozen",
            "type": "bool"
          },
          {
            "name": "status",
            "type": {
              "defined": "ReporterStatus"
            }
          },
          {
            "name": "role",
            "type": {
              "defined": "ReporterRole"
            }
          },
          {
            "name": "pubkey",
            "type": "publicKey"
          },
          {
            "name": "name",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "stake",
            "type": "u64"
          },
          {
            "name": "unlockEpoch",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "network",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "Account version"
            ],
            "type": "u16"
          },
          {
            "name": "community",
            "docs": [
              "Community account, which this network belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "name",
            "docs": [
              "Network name (i.e. ethereum, solana, near)"
            ],
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "schema",
            "type": {
              "defined": "NetworkSchema"
            }
          },
          {
            "name": "rewardMint",
            "docs": [
              "Reward token mint account"
            ],
            "type": "publicKey"
          },
          {
            "name": "addressTracerReward",
            "docs": [
              "Reward amount for tracers that report addresses to this network"
            ],
            "type": "u64"
          },
          {
            "name": "addressConfirmationReward",
            "docs": [
              "Reward amount for tracers and validators that confirm addresses on this network"
            ],
            "type": "u64"
          },
          {
            "name": "assetTracerReward",
            "docs": [
              "Reward amount for tracers that report assets to this network"
            ],
            "type": "u64"
          },
          {
            "name": "assetConfirmationReward",
            "docs": [
              "Reward amount for tracers and validators that confirm assets on this network"
            ],
            "type": "u64"
          },
          {
            "name": "replicationPrice",
            "docs": [
              "Replication price amount"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "reporter",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "Account version"
            ],
            "type": "u16"
          },
          {
            "name": "community",
            "docs": [
              "Community account, which this reporter belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "isFrozen",
            "docs": [
              "If this is true, reporter can't interact with the contract"
            ],
            "type": "bool"
          },
          {
            "name": "status",
            "docs": [
              "Reporter account status"
            ],
            "type": {
              "defined": "ReporterStatus"
            }
          },
          {
            "name": "role",
            "docs": [
              "Reporter's type"
            ],
            "type": {
              "defined": "ReporterRole"
            }
          },
          {
            "name": "pubkey",
            "docs": [
              "Reporter's wallet account"
            ],
            "type": "publicKey"
          },
          {
            "name": "name",
            "docs": [
              "Short reporter description"
            ],
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "stake",
            "docs": [
              "Current deposited stake"
            ],
            "type": "u64"
          },
          {
            "name": "unlockEpoch",
            "docs": [
              "Reporter can unstake at this epoch (0 if unstaking hasn't been requested)"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "reporterReward",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "Account version"
            ],
            "type": "u16"
          },
          {
            "name": "reporter",
            "docs": [
              "Reporter account to keep reward counter for"
            ],
            "type": "publicKey"
          },
          {
            "name": "network",
            "docs": [
              "Network that has the reward associated with"
            ],
            "type": "publicKey"
          },
          {
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "addressTracerCounter",
            "docs": [
              "Number of unclaimed address report rewards"
            ],
            "type": "u64"
          },
          {
            "name": "addressConfirmationCounter",
            "docs": [
              "Number of unclaimed address confirmation rewards"
            ],
            "type": "u64"
          },
          {
            "name": "assetTracerCounter",
            "docs": [
              "Number of unclaimed asset report rewards"
            ],
            "type": "u64"
          },
          {
            "name": "assetConfirmationCounter",
            "docs": [
              "Number of unclaimed asset confirmation rewards"
            ],
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "HapiEnvironment",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Devnet"
          },
          {
            "name": "Mainnet"
          }
        ]
      }
    },
    {
      "name": "Category",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "None"
          },
          {
            "name": "WalletService"
          },
          {
            "name": "MerchantService"
          },
          {
            "name": "MiningPool"
          },
          {
            "name": "Exchange"
          },
          {
            "name": "DeFi"
          },
          {
            "name": "OTCBroker"
          },
          {
            "name": "ATM"
          },
          {
            "name": "Gambling"
          },
          {
            "name": "IllicitOrganization"
          },
          {
            "name": "Mixer"
          },
          {
            "name": "DarknetService"
          },
          {
            "name": "Scam"
          },
          {
            "name": "Ransomware"
          },
          {
            "name": "Theft"
          },
          {
            "name": "Counterfeit"
          },
          {
            "name": "TerroristFinancing"
          },
          {
            "name": "Sanctions"
          },
          {
            "name": "ChildAbuse"
          },
          {
            "name": "Hacker"
          },
          {
            "name": "HighRiskJurisdiction"
          }
        ]
      }
    },
    {
      "name": "CaseStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Closed"
          },
          {
            "name": "Open"
          }
        ]
      }
    },
    {
      "name": "NetworkSchema",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Plain"
          },
          {
            "name": "Solana"
          },
          {
            "name": "Ethereum"
          },
          {
            "name": "Bitcoin"
          },
          {
            "name": "Near"
          }
        ]
      }
    },
    {
      "name": "ReporterStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Inactive"
          },
          {
            "name": "Active"
          },
          {
            "name": "Unstaking"
          }
        ]
      }
    },
    {
      "name": "ReporterRole",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Validator"
          },
          {
            "name": "Tracer"
          },
          {
            "name": "Publisher"
          },
          {
            "name": "Authority"
          },
          {
            "name": "Appraiser"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "UnexpectedAccount",
      "msg": "Unexpected account has been used"
    },
    {
      "code": 6001,
      "name": "Unauthorized",
      "msg": "Account is not authorized to perform this action"
    },
    {
      "code": 6002,
      "name": "NonSequentialCaseId",
      "msg": "Non-sequential case ID"
    },
    {
      "code": 6003,
      "name": "ReleaseEpochInFuture",
      "msg": "Release epoch is in future"
    },
    {
      "code": 6004,
      "name": "InvalidMint",
      "msg": "Invalid mint account"
    },
    {
      "code": 6005,
      "name": "InvalidReporter",
      "msg": "Invalid reporter account"
    },
    {
      "code": 6006,
      "name": "InactiveReporter",
      "msg": "Reporter account is not active"
    },
    {
      "code": 6007,
      "name": "InvalidToken",
      "msg": "Invalid token account"
    },
    {
      "code": 6008,
      "name": "CaseClosed",
      "msg": "Case closed"
    },
    {
      "code": 6009,
      "name": "InvalidReporterStatus",
      "msg": "Invalid reporter status"
    },
    {
      "code": 6010,
      "name": "AuthorityMismatch",
      "msg": "Authority mismatched"
    },
    {
      "code": 6011,
      "name": "CommunityMismatch",
      "msg": "Community mismatched"
    },
    {
      "code": 6012,
      "name": "FrozenReporter",
      "msg": "This reporter is frozen"
    },
    {
      "code": 6013,
      "name": "RiskOutOfRange",
      "msg": "Risk score must be in 0..10 range"
    },
    {
      "code": 6014,
      "name": "NetworkMismatch",
      "msg": "Network mismatched"
    },
    {
      "code": 6015,
      "name": "CaseMismatch",
      "msg": "Case mismatched"
    },
    {
      "code": 6016,
      "name": "SameCase",
      "msg": "Same address case"
    },
    {
      "code": 6017,
      "name": "NoReward",
      "msg": "There is no reward to claim"
    },
    {
      "code": 6018,
      "name": "IllegalOwner",
      "msg": "Account has illegal owner"
    },
    {
      "code": 6019,
      "name": "HighAccountRisk",
      "msg": "User account has high risk"
    },
    {
      "code": 6020,
      "name": "UnexpectedLength",
      "msg": "Unexpected account length"
    },
    {
      "code": 6021,
      "name": "InvalidAccountVersion",
      "msg": "Invalid account version"
    }
  ]
};
