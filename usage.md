## Instantiating
On Inj testnet, our codeid is 2670
On Inj mainnet, our codeid is [NOT UPLOADED]

# Actions
This contract at v0.1.0 has the following 4 actions:

## Adding Collection Metadata after-the-fact
After instantiation, the metadata may want to be updated for any number of reasons. 

## Listing tokens
to list a token, create a messsage list that looks like this:
```js
Messages: [
    MsgExecuteContract {
        sender: "token_owner",
        contract: "contract_address",
        funds: [],
        msg: {
            Approve: {
                spender: "exchange_address",
                token_id: "token_id_to_list",
                expires: 13591488142 // a few hundred years from now. Set as actual expirey if applicable 
            }
        }
    },
    MsgExecuteContract {
        sender: "token_owner",
        contract: "exchange_address",
        funds: [],
        msg: {
            List: {
                id: "token_id_to_list",
                price: 10000000000, // formula to get standard inj size is n / 10e18
                expires: 13591488142 // this timestamp will do as long as some climate theory is correct
            }
        }
    }
]
```
then sign and broadcast that message.  

## Buying tokens
Note: you can stack up to 10 messages to save gas. This is recommended for buying bulk.
message should be constructed as such:
```js
Messages: [
    MsgExecuteContract {
        sender: "buyer",
        contract: "exchange_contract",
        funds: [
            Coin {
                denom: "inj",
                amount: 10000000000 // 1 inj = 10e18
            }
        ],
        msg: {
            Buy: {
                token_id: "token_id_to_buy"
            }
        }
    }
]
```
then sign and broadcast.
Note: if too little INJ is provided, an Insufficient Funds error will follow.

## Delisting 
Obviously, the signer must be the token owner.
```js
Messages: [
    MsgExecuteContract {
        sender: "owner",
        contract: "exchange_contract",
        funds: [],
        msg: {
            Delist: {
                token_id: "token_id_to_delist"
            }
        }
    }
]
```

# Queries 
As well as the following 2 queries:

## GetMetadata
This query will return the metadata of the given collection. Because each collection has its own exchange address, this takes no arguments, and therefore the message will not be documented.

## GetListed
This query will return the listed NFTs from a given collection. Because each collection has its own exchange address, this takes no arguments, and therefore the message will not be documented.