use super::deserialize;
use resources::base64string::Base64String;
use resources::operation::Thresholds;
use resources::Balance;
use resources::Flags;
use resources::Signer;
use std::collections::HashMap;

/// In the Stellar network, users interact using accounts which can be controlled by a
/// corresponding keypair that can authorize transactions.
///
/// <https://www.stellar.org/developers/horizon/reference/resources/account.html>
#[derive(Deserialize, Debug)]
pub struct Account {
    id: String,
    paging_token: String,
    account_id: String,
    #[serde(deserialize_with = "deserialize::from_str")]
    sequence: u64,
    subentry_count: u64,
    last_modified_ledger: u64,
    thresholds: Thresholds,
    flags: Flags,
    balances: Vec<Balance>,
    signers: Vec<Signer>,
    data: HashMap<String, Base64String>,
}

impl Account {
    /// The canonical id of this account, suitable for use as the :id parameter
    /// for url templates that require an account’s ID. Returns a slice that lives
    /// as long as the account does.
    pub fn id_ref(&self) -> &str {
        &self.id
    }

    /// The account’s public key encoded into a base32 string representation.
    /// Returns a slice that lives as long as the account does.
    pub fn account_id_ref(&self) -> &str {
        &self.account_id
    }

    /// The canonical id of this account, suitable for use as the :id parameter
    /// for url templates that require an account’s ID.
    pub fn id(&self) -> &String {
        &self.id
    }

    /// The account’s public key encoded into a base32 string representation.
    pub fn account_id(&self) -> &String {
        &self.account_id
    }

    /// The current sequence number that can be used when submitting a transaction
    /// from this account.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// The number of account subentries.  This number is multiplied by
    /// 0.5 to determine the minimum required balance.
    pub fn subentry_count(&self) -> u64 {
        self.subentry_count
    }

    /// An array of the native asset or credits this account holds.
    pub fn balances(&self) -> &Vec<Balance> {
        &self.balances
    }

    /// An object of account flags.
    pub fn thresholds(&self) -> &Thresholds {
        &self.thresholds
    }

    /// The flags denote the enabling/disabling of certain asset issuer privileges.
    pub fn flags(&self) -> &Flags {
        &self.flags
    }

    /// An array of account signers with their weights.
    pub fn signers(&self) -> &Vec<Signer> {
        &self.signers
    }

    /// A key/value store of data attached to this account.
    pub fn data(&self) -> &HashMap<String, Base64String> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn asset_json() -> &'static str {
        include_str!("../../fixtures/account.json")
    }

    #[test]
    fn it_parses_account_from_json() {
        let account: Account = serde_json::from_str(&asset_json()).unwrap();
        assert_eq!(account.id, "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ");
        assert_eq!(account.paging_token, "");
        assert_eq!(account.account_id, "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ");
        assert_eq!(account.sequence, 604941848674305);
        assert_eq!(account.subentry_count, 1);
        assert_eq!(account.last_modified_ledger, 140917);
        assert_eq!(account.thresholds, Thresholds::new(0, 0, 0));
        assert!(!account.flags.is_auth_immutable());
        assert!(!account.flags.is_auth_required());
        assert!(!account.flags.is_auth_revocable());
        assert_eq!(
            account.balances[0],
            Balance::new(
                "100.0000000".to_string(),
                "0.0000000".to_string(),
                "0.0000000".to_string(),
                "100.0000000".to_string(),
                140993,
                "credit_alphanum4".to_string(),
                Some("USD".to_string()),
                Some("GBAUUA74H4XOQYRSOW2RZUA4QL5PB37U3JS5NE3RTB2ELJVMIF5RLMAG".to_string()),
            )
        );
        assert_eq!(
            account.signers,
            vec!(Signer::new(1, "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ".to_string(), "ed25519_public_key".to_string()))
        );
        assert_eq!(account.data, HashMap::new());
    }
}
