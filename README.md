# Deployed PSP34 example
This example is intended to help usage of PSP34 for commercial NFT projects.
This contract implements `openbrush`
- PSP34
- PSP34Enumerable
- PSP34Metadata
- Ownable

> Note! PSP34Mintable is overriden with empty implementation since it does not support payments

#### Custom trait implements
- fn mint_next(&mut self) -> Result<(), PSP34Error>;
- fn mint_for(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error>;
- fn set_base_uri(&mut self, uri: String) -> Result<(), PSP34Error>;
- fn token_uri(&self, token_id: u64) -> Result<String, PSP34Error>;
- fn max_supply(&self) -> u64;
- fn withdraw(&mut self) -> Result<(), PSP34Error>;

- [x] unit test is implemented
- [x] deployed on Shibuya
- [ ] deployed on Shiden

## Deployment on Shibuya
Contract address on Shibuya: 
`YSXjTTTiqYuUQT51WgMCQspKsw7qiY4Ng2Crp3Mc2hNmATc`

contract hash: 
`0x797865cd08843df1cc7668f66a2064bcc359fa633ceadb0bd65213e4f612a888`

## Deployment on Shiden
> soon
