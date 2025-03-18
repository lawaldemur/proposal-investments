/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/proposal_investment.json`.
 */
export type ProposalInvestment = {
  "address": "73QNc754LR5d8DiWFPWxyqMDUYkWvHni9NiHV16RHp26",
  "metadata": {
    "name": "proposalInvestment",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "acceptProposal",
      "docs": [
        "Only the company owner (via the Config account) can accept a proposal."
      ],
      "discriminator": [
        33,
        190,
        130,
        178,
        27,
        12,
        168,
        238
      ],
      "accounts": [
        {
          "name": "proposal",
          "writable": true
        },
        {
          "name": "config",
          "writable": true
        },
        {
          "name": "owner",
          "signer": true,
          "relations": [
            "config"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "createProposal",
      "docs": [
        "Anyone can create a new proposal."
      ],
      "discriminator": [
        132,
        116,
        68,
        174,
        216,
        160,
        198,
        22
      ],
      "accounts": [
        {
          "name": "proposal",
          "writable": true,
          "signer": true
        },
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "description",
          "type": "string"
        }
      ]
    },
    {
      "name": "distributeRewards",
      "docs": [
        "Distributes revenue rewards for a proposal.",
        "",
        "The company owner (or oracle acting on their behalf) supplies the total revenue amount",
        "that is held in the provided reward vault account. Then the instruction expects the remaining",
        "accounts to be provided in pairs: [investment account, corresponding investor wallet account].",
        "For each investment linked to this proposal, a reward is computed as:",
        "",
        "reward = (investment.amount / proposal.total_invested) * revenue_amount",
        "",
        "and the vault transfers that many lamports to the investor."
      ],
      "discriminator": [
        97,
        6,
        227,
        255,
        124,
        165,
        3,
        148
      ],
      "accounts": [
        {
          "name": "proposal",
          "writable": true
        },
        {
          "name": "config",
          "writable": true
        },
        {
          "name": "rewardVault",
          "writable": true
        },
        {
          "name": "owner",
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "revenueAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initialize",
      "docs": [
        "Initializes the Config account with the company owner."
      ],
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "signer": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "owner",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "invest",
      "docs": [
        "Anyone can invest in an existing proposal.",
        "The investor sends `amount` lamports which are transferred into the proposal account (acting as escrow).",
        "A separate Investment account is created to log the investment."
      ],
      "discriminator": [
        13,
        245,
        180,
        103,
        254,
        182,
        121,
        4
      ],
      "accounts": [
        {
          "name": "proposal",
          "writable": true
        },
        {
          "name": "investment",
          "writable": true,
          "signer": true
        },
        {
          "name": "investor",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "rejectProposal",
      "docs": [
        "Only the company owner can reject a proposal."
      ],
      "discriminator": [
        114,
        162,
        164,
        82,
        191,
        11,
        102,
        25
      ],
      "accounts": [
        {
          "name": "proposal",
          "writable": true
        },
        {
          "name": "config",
          "writable": true
        },
        {
          "name": "owner",
          "signer": true,
          "relations": [
            "config"
          ]
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "config",
      "discriminator": [
        155,
        12,
        170,
        224,
        30,
        250,
        204,
        130
      ]
    },
    {
      "name": "investment",
      "discriminator": [
        175,
        134,
        9,
        175,
        115,
        153,
        39,
        28
      ]
    },
    {
      "name": "proposal",
      "discriminator": [
        26,
        94,
        189,
        187,
        116,
        136,
        53,
        33
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "overflow",
      "msg": "Arithmetic overflow occurred."
    },
    {
      "code": 6001,
      "name": "invalidProposalStatus",
      "msg": "Proposal status does not allow this operation."
    },
    {
      "code": 6002,
      "name": "rewardsAlreadyDistributed",
      "msg": "Rewards have already been distributed for this proposal."
    },
    {
      "code": 6003,
      "name": "insufficientVaultBalance",
      "msg": "The reward vault does not have enough funds."
    },
    {
      "code": 6004,
      "name": "invalidOwner",
      "msg": "Invalid owner for the investment account."
    }
  ],
  "types": [
    {
      "name": "config",
      "docs": [
        "Config account holds the company owner."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "investment",
      "docs": [
        "Investment account records an individual investment."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposal",
            "type": "pubkey"
          },
          {
            "name": "investor",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "proposal",
      "docs": [
        "Proposal account holds the proposal data."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "description",
            "type": "string"
          },
          {
            "name": "status",
            "type": {
              "defined": {
                "name": "proposalStatus"
              }
            }
          },
          {
            "name": "totalInvested",
            "type": "u64"
          },
          {
            "name": "rewardsDistributed",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "proposalStatus",
      "docs": [
        "Enum for the proposal status."
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "pending"
          },
          {
            "name": "accepted"
          },
          {
            "name": "rejected"
          }
        ]
      }
    }
  ]
};
