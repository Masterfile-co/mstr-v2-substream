    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's functions.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod functions {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct FactoryType {}
        impl FactoryType {
            const METHOD_ID: [u8; 4] = [10u8, 133u8, 240u8, 203u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[u8; 32usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[u8; 32usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for FactoryType {
            const NAME: &'static str = "FACTORY_TYPE";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[u8; 32usize]> for FactoryType {
            fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Version {}
        impl Version {
            const METHOD_ID: [u8; 4] = [255u8, 161u8, 173u8, 116u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<substreams::scalar::BigInt, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<substreams::scalar::BigInt> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Version {
            const NAME: &'static str = "VERSION";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<substreams::scalar::BigInt>
        for Version {
            fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct DeployDrop {
            pub salt: [u8; 32usize],
            pub details: (
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                Vec<u8>,
                Vec<u8>,
                Vec<u8>,
                [u8; 32usize],
            ),
            pub editions: Vec<
                (
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    [u8; 32usize],
                ),
            >,
            pub mystery_box: (
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
            ),
            pub bundle: (
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                Vec<substreams::scalar::BigInt>,
            ),
        }
        impl DeployDrop {
            const METHOD_ID: [u8; 4] = [60u8, 180u8, 215u8, 38u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Uint(64usize),
                                    ethabi::ParamType::Uint(64usize),
                                    ethabi::ParamType::Uint(16usize),
                                    ethabi::ParamType::Address, ethabi::ParamType::Address,
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::FixedBytes(32usize)
                                ],
                            ),
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Uint(64usize),
                                            ethabi::ParamType::Uint(64usize),
                                            ethabi::ParamType::Uint(16usize),
                                            ethabi::ParamType::FixedBytes(32usize)
                                        ],
                                    ),
                                ),
                            ),
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(8usize)
                                ],
                            ),
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize)))
                                ],
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    salt: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    details: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[0usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[1usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[2usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            tuple_elements[3usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[4usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[5usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            {
                                let mut result = [0u8; 32];
                                let v = tuple_elements[6usize]
                                    .clone()
                                    .into_fixed_bytes()
                                    .expect(INTERNAL_ERR);
                                result.copy_from_slice(&v);
                                result
                            },
                        )
                    },
                    editions: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut result = [0u8; 32];
                                    let v = tuple_elements[3usize]
                                        .clone()
                                        .into_fixed_bytes()
                                        .expect(INTERNAL_ERR);
                                    result.copy_from_slice(&v);
                                    result
                                },
                            )
                        })
                        .collect(),
                    mystery_box: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[0usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[1usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[2usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                        )
                    },
                    bundle: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[0usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[1usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            tuple_elements[2usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let mut v = [0 as u8; 32];
                                    inner
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                })
                                .collect(),
                        )
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::FixedBytes(self.salt.as_ref().to_vec()),
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .details.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .details.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .details.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .details.3)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .details.4)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .details.5)), ethabi::Token::FixedBytes(self.details.6
                                .as_ref().to_vec())
                            ],
                        ),
                        {
                            let v = self
                                .editions
                                .iter()
                                .map(|inner| ethabi::Token::Tuple(
                                    vec![
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::FixedBytes(inner.3.as_ref()
                                        .to_vec())
                                    ],
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .mystery_box.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .mystery_box.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .mystery_box.2.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)
                            ],
                        ),
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .bundle.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .bundle.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), { let v = self.bundle.2.iter().map(| inner
                                | ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)).collect(); ethabi::Token::Array(v) }
                            ],
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for DeployDrop {
            const NAME: &'static str = "deployDrop";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for DeployDrop {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PredictDropAddress {
            pub salt: [u8; 32usize],
        }
        impl PredictDropAddress {
            const METHOD_ID: [u8; 4] = [116u8, 179u8, 202u8, 241u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    salt: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[ethabi::Token::FixedBytes(self.salt.as_ref().to_vec())],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for PredictDropAddress {
            const NAME: &'static str = "predictDropAddress";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for PredictDropAddress {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Registry {}
        impl Registry {
            const METHOD_ID: [u8; 4] = [123u8, 16u8, 57u8, 153u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Registry {
            const NAME: &'static str = "registry";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for Registry {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDrop {
            pub drop: Vec<u8>,
        }
        impl SetDrop {
            const METHOD_ID: [u8; 4] = [248u8, 178u8, 70u8, 38u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    drop: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[ethabi::Token::Address(ethabi::Address::from_slice(&self.drop))],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetDrop {
            const NAME: &'static str = "setDrop";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUsdc {
            pub usdc: Vec<u8>,
        }
        impl SetUsdc {
            const METHOD_ID: [u8; 4] = [203u8, 20u8, 104u8, 121u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    usdc: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[ethabi::Token::Address(ethabi::Address::from_slice(&self.usdc))],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetUsdc {
            const NAME: &'static str = "setUsdc";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
    }
    /// Contract's events.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod events {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct DropDeployed {
            pub drop: Vec<u8>,
            pub safe: Vec<u8>,
        }
        impl DropDeployed {
            const TOPIC_ID: [u8; 32] = [
                127u8,
                147u8,
                38u8,
                54u8,
                54u8,
                124u8,
                205u8,
                238u8,
                47u8,
                151u8,
                164u8,
                249u8,
                234u8,
                77u8,
                63u8,
                76u8,
                47u8,
                152u8,
                112u8,
                232u8,
                18u8,
                11u8,
                107u8,
                208u8,
                89u8,
                196u8,
                238u8,
                12u8,
                227u8,
                183u8,
                129u8,
                176u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    drop: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'drop' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    safe: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'safe' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for DropDeployed {
            const NAME: &'static str = "DropDeployed";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
    }