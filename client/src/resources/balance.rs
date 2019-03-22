/// Balances Object
#[derive(Deserialize, Debug, PartialEq)]
pub struct Balance {
    balance: String,
    #[serde(default)]
    buying_liabilities: String,
    #[serde(default)]
    selling_liabilities: String,
    #[serde(default)]
    limit: String,
    #[serde(default)]
    last_modified_ledger: u64,
    asset_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_issuer: Option<String>,
}

impl Balance {
    /// create a balance
    pub fn new(
        balance: String,
        buying_liabilities: String,
        selling_liabilities: String,
        limit: String,
        last_modified_ledger: u64,
        asset_type: String,
        asset_code: Option<String>,
        asset_issuer: Option<String>,
    ) -> Balance {
        Balance {
            balance,
            buying_liabilities,
            selling_liabilities,
            limit,
            last_modified_ledger,
            asset_type,
            asset_code,
            asset_issuer,
        }
    }
    /// How much of an asset is owned.
    pub fn balance(&self) -> &String {
        &self.balance
    }
    /// The total amount of an asset offered to buy aggregated over all offers owned by this account.
    pub fn buying_liabilities(&self) -> &String {
        &self.buying_liabilities
    }
    /// The total amount of an asset offered to sell aggregated over all offers owned by this account.
    pub fn selling_liabilities(&self) -> &String {
        &self.selling_liabilities
    }
    /// The maximum amount of an asset that this account is willing to accept (this is specified when an account opens a trustline).
    pub fn limit(&self) -> &String {
        &self.limit
    }
    /// last modified
    pub fn last_modified_ledger(&self) -> &u64 {
        &self.last_modified_ledger
    }
    /// Either native, credit_alphanum4, or credit_alphanum12.
    pub fn asset_type(&self) -> &String {
        &self.asset_type
    }
    /// The code for the asset.
    pub fn asset_code(&self) -> &Option<String> {
        &self.asset_code
    }
    /// The stellar address of the given asset’s issuer.
    pub fn asset_issuer(&self) -> &Option<String> {
        &self.asset_issuer
    }
}
