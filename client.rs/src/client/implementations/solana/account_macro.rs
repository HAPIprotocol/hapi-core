#[macro_export]
macro_rules! get_solana_account {
    ($self:expr, $address:expr, $account:ident) => {
        <$account>::try_from(
            $self
                .get_account_data::<hapi_core_solana::$account>($address)
                .await?,
        )
    };
}

#[macro_export]
macro_rules! get_solana_accounts {
    ($self:expr, $account:ident) => {{
        let data = $self
            .get_accounts::<hapi_core_solana::$account>(hapi_core_solana::$account::LEN)
            .await?;

        let mut result: Vec<$account> = vec![];

        for (_, acc) in data {
            if acc.network == $self.network {
                result.push(<$account>::try_from(acc)?);
            }
        }

        Ok(result)
    }};
}

#[macro_export]
macro_rules! get_solana_account_count {
    ($self:expr, $account:ident) => {{
        let accounts: Result<Vec<$account>> = get_solana_accounts!($self, $account);
        Ok(accounts?.len() as u64)
    }};
}
