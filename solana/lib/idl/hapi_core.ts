export type HapiCore = {
  "version": "0.1.0",
  "name": "hapi_core",
  "instructions": [
    {
      "name": "createNetwork",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "network",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeTokenAccount",
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
          "name": "stakeInfo",
          "type": {
            "defined": "StakeConfiguration"
          }
        },
        {
          "name": "rewardInfo",
          "type": {
            "defined": "RewardConfiguration"
          }
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "updateConfiguration",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "network",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "stakeInfo",
          "type": {
            "defined": "StakeConfiguration"
          }
        },
        {
          "name": "rewardInfo",
          "type": {
            "defined": "RewardConfiguration"
          }
        }
      ]
    },
    {
      "name": "setNetworkAuthority",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "network",
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
    }
  ],
  "accounts": [
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
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "authority",
            "docs": [
              "Network authority"
            ],
            "type": "publicKey"
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
            "docs": [
              "Network address schema"
            ],
            "type": {
              "defined": "NetworkSchema"
            }
          },
          {
            "name": "stakeMint",
            "docs": [
              "Stake token mint account"
            ],
            "type": "publicKey"
          },
          {
            "name": "stakeInfo",
            "docs": [
              "Stake configuration info"
            ],
            "type": {
              "defined": "StakeConfiguration"
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
            "name": "rewardInfo",
            "docs": [
              "Reward configuration info"
            ],
            "type": {
              "defined": "RewardConfiguration"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "StakeConfiguration",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "unlockDuration",
            "docs": [
              "Duration in seconds of reporter suspension before the stake can be withdrawn"
            ],
            "type": "u64"
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
            "name": "publisherStake",
            "docs": [
              "Amount of stake required from a reporter of publisher type"
            ],
            "type": "u64"
          },
          {
            "name": "authorityStake",
            "docs": [
              "Amount of stake required from a reporter of authority type"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "RewardConfiguration",
      "type": {
        "kind": "struct",
        "fields": [
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
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidToken",
      "msg": "Invalid token account"
    },
    {
      "code": 6001,
      "name": "AuthorityMismatch",
      "msg": "Authority mismatched"
    },
    {
      "code": 6002,
      "name": "IllegalOwner",
      "msg": "Account has illegal owner"
    }
  ]
};

export const IDL: HapiCore = {
  "version": "0.1.0",
  "name": "hapi_core",
  "instructions": [
    {
      "name": "createNetwork",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "network",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeTokenAccount",
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
          "name": "stakeInfo",
          "type": {
            "defined": "StakeConfiguration"
          }
        },
        {
          "name": "rewardInfo",
          "type": {
            "defined": "RewardConfiguration"
          }
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "updateConfiguration",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "network",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "stakeInfo",
          "type": {
            "defined": "StakeConfiguration"
          }
        },
        {
          "name": "rewardInfo",
          "type": {
            "defined": "RewardConfiguration"
          }
        }
      ]
    },
    {
      "name": "setNetworkAuthority",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "network",
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
    }
  ],
  "accounts": [
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
            "name": "bump",
            "docs": [
              "Seed bump for PDA"
            ],
            "type": "u8"
          },
          {
            "name": "authority",
            "docs": [
              "Network authority"
            ],
            "type": "publicKey"
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
            "docs": [
              "Network address schema"
            ],
            "type": {
              "defined": "NetworkSchema"
            }
          },
          {
            "name": "stakeMint",
            "docs": [
              "Stake token mint account"
            ],
            "type": "publicKey"
          },
          {
            "name": "stakeInfo",
            "docs": [
              "Stake configuration info"
            ],
            "type": {
              "defined": "StakeConfiguration"
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
            "name": "rewardInfo",
            "docs": [
              "Reward configuration info"
            ],
            "type": {
              "defined": "RewardConfiguration"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "StakeConfiguration",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "unlockDuration",
            "docs": [
              "Duration in seconds of reporter suspension before the stake can be withdrawn"
            ],
            "type": "u64"
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
            "name": "publisherStake",
            "docs": [
              "Amount of stake required from a reporter of publisher type"
            ],
            "type": "u64"
          },
          {
            "name": "authorityStake",
            "docs": [
              "Amount of stake required from a reporter of authority type"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "RewardConfiguration",
      "type": {
        "kind": "struct",
        "fields": [
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
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidToken",
      "msg": "Invalid token account"
    },
    {
      "code": 6001,
      "name": "AuthorityMismatch",
      "msg": "Authority mismatched"
    },
    {
      "code": 6002,
      "name": "IllegalOwner",
      "msg": "Account has illegal owner"
    }
  ]
};
