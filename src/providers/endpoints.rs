use super::{Endpoint, Region};

//tip accounts

pub const JITO_TIP_ACCOUNTS: &[&str] = &[
    "96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5",
    "HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe",
    "Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY",
    "ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49",
    "DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh",
    "ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt",
    "DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL",
    "3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT",
];

pub const TEMPORAL_TIP_ACCOUNTS: &[&str] = &[
    "TEMPaMeCRFAS9EKF53Jd6KpHxgL47uWLcpFArU1Fanq",
    "noz3jAjPiHuBPqiSPkkugaJDkJscPuRhYnSpbi8UvC4",
    "noz3str9KXfpKknefHji8L1mPgimezaiUyCHYMDv1GE",
    "noz6uoYCDijhu1V7cutCpwxNiSovEwLdRHPwmgCGDNo",
    "noz9EPNcT7WH6Sou3sr3GGjHQYVkN3DNirpbvDkv9YJ",
    "nozc5yT15LazbLTFVZzoNZCwjh3yUtW86LoUyqsBu4L",
    "nozFrhfnNGoyqwVuwPAW4aaGqempx4PU6g6D9CJMv7Z",
    "nozievPk7HyK1Rqy1MPJwVQ7qQg2QoJGyP71oeDwbsu",
    "noznbgwYnBLDHu8wcQVCEw6kDrXkPdKkydGJGNXGvL7",
    "nozNVWs5N8mgzuD3qigrCG2UoKxZttxzZ85pvAQVrbP",
    "nozpEGbwx4BcGp6pvEdAh1JoC2CQGZdU6HbNP1v2p6P",
    "nozrhjhkCr3zXT3BiT4WCodYCUFeQvcdUkM7MqhKqge",
    "nozrwQtWhEdrA6W8dkbt9gnUaMs52PdAv5byipnadq3",
    "nozUacTVWub3cL4mJmGCYjKZTnE9RbdY5AP46iQgbPJ",
    "nozWCyTPppJjRuw2fpzDhhWbW355fzosWSzrrMYB1Qk",
    "nozWNju6dY353eMkMqURqwQEoM3SFgEKC6psLCSfUne",
    "nozxNBgWohjR75vdspfxR5H9ceC7XXH99xpxhVGt3Bb",
];

pub const ZEROSLOT_TIP_ACCOUNTS: &[&str] = &[
    "6fQaVhYZA4w3MBSXjJ81Vf6W1EDYeUPXpgVQ6UQyU1Av",
    "4HiwLEP2Bzqj3hM2ENxJuzhcPCdsafwiet3oGkMkuQY4",
    "7toBU3inhmrARGngC7z6SjyP85HgGMmCTEwGNRAcYnEK",
    "8mR3wB1nh4D6J9RUCugxUpc6ya8w38LPxZ3ZjcBhgzws",
    "6SiVU5WEwqfFapRuYCndomztEwDjvS5xgtEof3PLEGm9",
    "TpdxgNJBWZRL8UXF5mrEsyWxDWx9HQexA9P1eTWQ42p",
    "D8f3WkQu6dCF33cZxuAsrKHrGsqGP2yvAHf8mX6RXnwf",
    "GQPFicsy3P3NXxB5piJohoxACqTvWE9fKpLgdsMduoHE",
    "Ey2JEr8hDkgN8qKJGrLf2yFjRhW7rab99HVxwi5rcvJE",
    "4iUgjMT8q2hNZnLuhpqZ1QtiV8deFPy2ajvvjEpKKgsS",
    "3Rz8uD83QsU8wKvZbgWAPvCNDU6Fy8TSZTMcPm3RB6zt",
];

pub const ASTRALANE_TIP_ACCOUNTS: &[&str] = &[
    "astrazznxsGUhWShqgNtAdfrzP2G83DzcWVJDxwV9bF",
    "astra4uejePWneqNaJKuFFA8oonqCE1sqF6b45kDMZm",
    "astra9xWY93QyfG6yM8zwsKsRodscjQ2uU2HKNL5prk",
    "astraRVUuTHjpwEVvNBeQEgwYx9w9CFyfxjYoobCZhL",
    "astraEJ2fEj8Xmy6KLG7B3VfbKfsHXhHrNdCQx7iGJK",
    "astraubkDw81n4LuutzSQ8uzHCv4BhPVhfvTcYv8SKC",
    "astraZW5GLFefxNPAatceHhYjfA1ciq9gvfEg2S47xk",
    "astrawVNP4xDBKT7rAdxrLYiTSTdqtUr63fSMduivXK",
];

pub const STELLIUM_TIP_ACCOUNTS: &[&str] = &[
    "ste11JV3MLMM7x7EJUM2sXcJC1H7F4jBLnP9a9PG8PH",
    "ste11MWPjXCRfQryCshzi86SGhuXjF4Lv6xMXD2AoSt",
    "ste11p5x8tJ53H1NbNQsRBg1YNRd4GcVpxtDw8PBpmb",
    "ste11p7e2KLYou5bwtt35H7BM6uMdo4pvioGjJXKFcN",
    "ste11TMV68LMi1BguM4RQujtbNCZvf1sjsASpqgAvSX",
];

pub const HELIUS_TIP_ACCOUNTS: &[&str] = &[
    "4ACfpUFoaSD9bfPdeu6DBt89gB6ENTeHBXCAi87NhDEE",
    "D2L6yPZ2FmmmTKPgzaMKdhu6EWZcTpLy1Vhx8uvZe7NZ",
    "9bnz4RShgq1hAnLnZbP8kbgBg1kEmcJBYQq3gQbmnSta",
    "5VY91ws6B2hMmBFRsXkoAAdsPHBJwRfBht4DXox3xkwn",
    "2nyhqdwKcJZR2vcqCyrYsaPVdAnFoJjiksCXJ7hfEYgD",
    "2q5pghRs6arqVjRvT5gfgWfWcHWmw1ZuCzphgd5KfWGJ",
    "wyvPkWjVZz1M8fHQnMMCDTQDbkManefNNhweYk5WkcF",
    "3KCKozbAaF75qEU33jtzozcJ29yJuaLJTy2jFdzUY8bT",
    "4vieeGHPYPG2MmyPRcYjdiDmmhN3ww7hsFNap8pVN3Ey",
    "4TQLFNWK8AovT1gFvda5jfw2oJeRMKEmw7aH6MGBJ3or",
];

//endpoints

pub const JITO_ENDPOINTS: &[Endpoint] = &[
        Endpoint { url: "https://mainnet.block-engine.jito.wtf",      region: Region::UsEast,        label: "Global",     rpc_method: "sendBundle" },
        Endpoint { url: "https://ny.mainnet.block-engine.jito.wtf",   region: Region::UsEast,        label: "NY",         rpc_method: "sendBundle" },
        Endpoint { url: "https://slc.mainnet.block-engine.jito.wtf",  region: Region::UsWest,        label: "SLC",        rpc_method: "sendBundle" },
        Endpoint { url: "https://amsterdam.mainnet.block-engine.jito.wtf", region: Region::EuWest,   label: "Amsterdam",  rpc_method: "sendBundle" },
        Endpoint { url: "https://dublin.mainnet.block-engine.jito.wtf",    region: Region::EuWest,   label: "Dublin",     rpc_method: "sendBundle" },
        Endpoint { url: "https://london.mainnet.block-engine.jito.wtf",    region: Region::EuWest,   label: "London",     rpc_method: "sendBundle" },
        Endpoint { url: "https://frankfurt.mainnet.block-engine.jito.wtf", region: Region::EuCentral, label: "Frankfurt", rpc_method: "sendBundle" },
        Endpoint { url: "https://tokyo.mainnet.block-engine.jito.wtf",     region: Region::AsiaTokyo,     label: "Tokyo",     rpc_method: "sendBundle" },
        Endpoint { url: "https://singapore.mainnet.block-engine.jito.wtf", region: Region::AsiaSingapore, label: "Singapore", rpc_method: "sendBundle" },
];

pub const TEMPORAL_ENDPOINTS: &[Endpoint] = &[
        Endpoint { url: "http://pit1.nozomi.temporal.xyz/", region: Region::UsEast,        label: "Pittsburgh", rpc_method: "sendTransaction" },
        Endpoint { url: "http://ewr1.nozomi.temporal.xyz/", region: Region::UsEast,        label: "Newark",     rpc_method: "sendTransaction" },
        Endpoint { url: "http://ams1.nozomi.temporal.xyz/", region: Region::EuWest,        label: "Amsterdam",  rpc_method: "sendTransaction" },
        Endpoint { url: "http://fra2.nozomi.temporal.xyz/", region: Region::EuCentral,     label: "Frankfurt",  rpc_method: "sendTransaction" },
        Endpoint { url: "http://tyo1.nozomi.temporal.xyz/", region: Region::AsiaTokyo,     label: "Tokyo",      rpc_method: "sendTransaction" },
        Endpoint { url: "http://sgp1.nozomi.temporal.xyz/", region: Region::AsiaSingapore, label: "Singapore",  rpc_method: "sendTransaction" },
];

pub const ZEROSLOT_ENDPOINTS: &[Endpoint] = &[
        Endpoint { url: "https://ny.0slot.trade",  region: Region::UsEast,        label: "NY",        rpc_method: "sendTransaction" },
        Endpoint { url: "https://la.0slot.trade",  region: Region::UsWest,        label: "LA",        rpc_method: "sendTransaction" },
        Endpoint { url: "https://ams.0slot.trade", region: Region::EuWest,        label: "Amsterdam", rpc_method: "sendTransaction" },
        Endpoint { url: "https://de1.0slot.trade", region: Region::EuCentral,     label: "Frankfurt", rpc_method: "sendTransaction" },
        Endpoint { url: "https://jp.0slot.trade",  region: Region::AsiaTokyo,     label: "Tokyo",     rpc_method: "sendTransaction" },
];

pub const ASTRALANE_ENDPOINTS: &[Endpoint] = &[
        Endpoint { url: "http://ny.gateway.astralane.io/iris",  region: Region::UsEast,        label: "NY",        rpc_method: "sendTransaction" },
        Endpoint { url: "http://lax.gateway.astralane.io/iris", region: Region::UsWest,        label: "LAX",       rpc_method: "sendTransaction" },
        Endpoint { url: "http://ams.gateway.astralane.io/iris", region: Region::EuWest,        label: "Amsterdam", rpc_method: "sendTransaction" },
        Endpoint { url: "http://fr.gateway.astralane.io/iris",  region: Region::EuCentral,     label: "Frankfurt", rpc_method: "sendTransaction" },
        Endpoint { url: "http://jp.gateway.astralane.io/iris",  region: Region::AsiaTokyo,     label: "Tokyo",     rpc_method: "sendTransaction" },
];

pub const STELLIUM_ENDPOINTS: &[Endpoint] = &[
        Endpoint { url: "http://ewr1.flashrpc.com", region: Region::UsEast,        label: "Newark",    rpc_method: "sendTransaction" },
        Endpoint { url: "http://ams1.flashrpc.com", region: Region::EuWest,        label: "Amsterdam", rpc_method: "sendTransaction" },
        Endpoint { url: "http://lhr1.flashrpc.com", region: Region::EuWest,        label: "London",    rpc_method: "sendTransaction" },
        Endpoint { url: "http://fra1.flashrpc.com", region: Region::EuCentral,     label: "Frankfurt", rpc_method: "sendTransaction" },
        Endpoint { url: "http://tyo1.flashrpc.com", region: Region::AsiaTokyo,     label: "Tokyo",     rpc_method: "sendTransaction" },
];

/// NOTE !!! Todo: Need to create a helius account to get all endpoints and tips too. 
pub const HELIUS_ENDPOINTS: &[Endpoint] = &[
        Endpoint { url: "https://mainnet.helius-rpc.com", region: Region::UsEast, label: "EWR", rpc_method: "sendTransaction" },
];


pub const BAM_ENDPOINTS: &[Endpoint] = &[
        Endpoint { url: "https://ny.mainnet.bam.jito.wtf",          region: Region::UsEast,        label: "NY",         rpc_method: "sendBundle" },
        Endpoint { url: "https://pittsburgh.mainnet.bam.jito.wtf",   region: Region::UsEast,        label: "Pittsburgh", rpc_method: "sendBundle" },
        Endpoint { url: "https://slc.mainnet.bam.jito.wtf",          region: Region::UsWest,        label: "SLC",        rpc_method: "sendBundle" },
        Endpoint { url: "https://lax.mainnet.bam.jito.wtf",          region: Region::UsWest,        label: "LAX",        rpc_method: "sendBundle" },
        Endpoint { url: "https://dallas.mainnet.bam.jito.wtf",       region: Region::UsWest,        label: "Dallas",     rpc_method: "sendBundle" },
        Endpoint { url: "https://amsterdam.mainnet.bam.jito.wtf",    region: Region::EuWest,        label: "Amsterdam",  rpc_method: "sendBundle" },
        Endpoint { url: "https://dublin.mainnet.bam.jito.wtf",       region: Region::EuWest,        label: "Dublin",     rpc_method: "sendBundle" },
        Endpoint { url: "https://london.mainnet.bam.jito.wtf",       region: Region::EuWest,        label: "London",     rpc_method: "sendBundle" },
        Endpoint { url: "https://frankfurt.mainnet.bam.jito.wtf",    region: Region::EuCentral,     label: "Frankfurt",  rpc_method: "sendBundle" },
        Endpoint { url: "https://tokyo.mainnet.bam.jito.wtf",        region: Region::AsiaTokyo,     label: "Tokyo",      rpc_method: "sendBundle" },
        Endpoint { url: "https://singapore.mainnet.bam.jito.wtf",    region: Region::AsiaSingapore, label: "Singapore",  rpc_method: "sendBundle" },
];

pub const HARMONIC_ENDPOINTS: &[Endpoint] = &[
        Endpoint { url: "https://ewr.auction.harmonic.gg", region: Region::UsEast,        label: "Newark",    rpc_method: "sendBundle" },
        Endpoint { url: "https://ams.auction.harmonic.gg", region: Region::EuWest,        label: "Amsterdam", rpc_method: "sendBundle" },
        Endpoint { url: "https://lon.auction.harmonic.gg", region: Region::EuWest,        label: "London",    rpc_method: "sendBundle" },
        Endpoint { url: "https://fra.auction.harmonic.gg", region: Region::EuCentral,     label: "Frankfurt", rpc_method: "sendBundle" },
        Endpoint { url: "https://tyo.auction.harmonic.gg", region: Region::AsiaTokyo,     label: "Tokyo",     rpc_method: "sendBundle" },
        Endpoint { url: "https://sgp.auction.harmonic.gg", region: Region::AsiaSingapore, label: "Singapore", rpc_method: "sendBundle" },
];