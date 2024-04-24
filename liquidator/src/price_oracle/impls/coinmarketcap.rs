pub struct CoinMarketCap {
    api_key: String,
}

impl PriceOracle for CoinMarketCap {
    async fn get_prices(&self, _addresses: Vec<Address>) -> Result<Vec<(Address, ScaledNum)>> {
        // this is hardcoded because I didn't feel like navigating CMC's painfuls docs or
        // having to debug & deserialize multiple API calls
        // if this is a decent price oracle then maybe I'll actually put effort into this function but it's late
        // let tokens_and_id = vec![
        //     // ctoken address, CMC id
        //     ("0xf7B5965f5C117Eb1B5450187c9DcFccc3C317e8E", 2396), //WETH
        //     ("0xEC8FEa79026FfEd168cCf5C627c7f486D77b765F", 3408), // USDC
        //     ("0x5Ff29E4470799b982408130EFAaBdeeAE7f66a10", 825),  // USDT
        //     ("0x5569b83de187375d43FBd747598bfe64fC8f6436", 4943), // DAI
        //     ("0x8cD6b19A07d754bF36AdEEE79EDF4F2134a8F571", 11840), // OP
        //     ("0xd14451E0Fa44B18f08aeB1E4a4d092B823CaCa68", 2927), // SUSD
        //     ("0xD7dAabd899D1fAbbC3A9ac162568939CEc0393Cc", 2586), // SNX
        //     ("0x33865E09A572d4F1CC4d75Afc9ABcc5D3d4d867D", 3717), // WBTC
        //     ("0xAFdf91f120DEC93c65fd63DBD5ec372e5dcA5f82", 9566), // LUSD
        //     ("0x26AaB17f27CD1c8d06a0Ad8E4a1Af8B1032171d5", 21535), // wstETH (using id for coinbase wrapped staked eth)
        //     ("0xE7De932d50EfC9ea0a7a409Fc015B4f71443528e", 10238), // MAI
        //     ("0x1AfD1fF9E441973B7D34c7B8AbE91d94F1B23ce0", 3408),  // USDCnative (using id for usdc)
        // ];

        // let endpoint

        todo!();
    }
}
