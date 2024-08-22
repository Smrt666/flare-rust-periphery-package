// This file was generated automatically. Do not edit manually.
use flare_rust_periphery_package::{songbird, interface_to_abi, name_to_abi, Chain::Songbird};

#[test]
fn abis_exist_for_all_contracts() {
    let _ = songbird::interface_abis::IWNat;
    let _ = songbird::interface_abis::IDelegationAccount;
    let _ = songbird::interface_abis::ICChainStake;
    let _ = songbird::interface_abis::IPriceSubmitter;
    let _ = songbird::interface_abis::IFastUpdateIncentiveManager;
    let _ = songbird::interface_abis::IFtsoFeedIdConverter;
    let _ = songbird::interface_abis::ICChainVotePower;
    let _ = songbird::interface_abis::IRNatAccount;
    let _ = songbird::interface_abis::IEVMTransactionVerification;
    let _ = songbird::interface_abis::IFlareContractRegistry;
    let _ = songbird::interface_abis::IGovernanceSettings;
    let _ = songbird::interface_abis::IRandomProvider;
    let _ = songbird::interface_abis::IValidatorRewardOffersManager;
    let _ = songbird::interface_abis::EVMTransaction;
    let _ = songbird::interface_abis::IPaymentVerification;
    let _ = songbird::interface_abis::IVPToken;
    let _ = songbird::interface_abis::IFtsoInflationConfigurations;
    let _ = songbird::interface_abis::IGovernanceVotePower;
    let _ = songbird::interface_abis::ISubmission;
    let _ = songbird::interface_abis::IBalanceDecreasingTransactionVerification;
    let _ = songbird::interface_abis::IFtsoManager;
    let _ = songbird::interface_abis::IEntityManager;
    let _ = songbird::interface_abis::IPollingFtso;
    let _ = songbird::interface_abis::IFlareSystemsCalculator;
    let _ = songbird::interface_abis::IFtsoFeedDecimals;
    let _ = songbird::interface_abis::IReferencedPaymentNonexistenceVerification;
    let _ = songbird::interface_abis::FlareContractsRegistryLibrary;
    let _ = songbird::interface_abis::BalanceDecreasingTransaction;
    let _ = songbird::interface_abis::IIncreaseManager;
    let _ = songbird::interface_abis::IMerkleRootStorage;
    let _ = songbird::interface_abis::IStateConnector;
    let _ = songbird::interface_abis::IFtsoRewardOffersManager;
    let _ = songbird::interface_abis::IClaimSetupManager;
    let _ = songbird::interface_abis::IConfirmedBlockHeightExistsVerification;
    let _ = songbird::interface_abis::IAddressValidityVerification;
    let _ = songbird::interface_abis::ReferencedPaymentNonexistence;
    let _ = songbird::interface_abis::IFastUpdatesConfiguration;
    let _ = songbird::interface_abis::IGenericRewardManager;
    let _ = songbird::interface_abis::IFlareSystemsManager;
    let _ = songbird::interface_abis::IFlareAssetRegistry;
    let _ = songbird::interface_abis::IGovernor;
    let _ = songbird::interface_abis::IFtsoRewardManager;
    let _ = songbird::interface_abis::IVoterRegistry;
    let _ = songbird::interface_abis::AddressValidity;
    let _ = songbird::interface_abis::IWNatDelegationFee;
    let _ = songbird::interface_abis::Payment;
    let _ = songbird::interface_abis::IVoterWhitelister;
    let _ = songbird::interface_abis::IFtsoFeedPublisher;
    let _ = songbird::interface_abis::IRelay;
    let _ = songbird::interface_abis::IRewardManager;
    let _ = songbird::interface_abis::IFastUpdater;
    let _ = songbird::interface_abis::IRNat;
    let _ = songbird::interface_abis::IVPContractEvents;
    let _ = songbird::interface_abis::IFtso;
    let _ = songbird::interface_abis::ConfirmedBlockHeightExists;
    let _ = songbird::interface_abis::IFtsoRegistry;
    let _ = songbird::interface_abis::IFtsoGenesis;
    let _ = songbird::interface_abis::IFlareDaemonize;
    let _ = songbird::interface_abis::IFtsoRegistryGenesis;
    let _ = songbird::interface_abis::IFtsoManagerGenesis;
    let _ = songbird::interface_abis::IIEntityManager;
    let _ = songbird::interface_abis::IICleanupBlockNumberManager;
    let _ = songbird::interface_abis::IIRewardManager;
    let _ = songbird::interface_abis::IIVoterRegistry;
    let _ = songbird::interface_abis::IIPublicKeyVerifier;
    let _ = songbird::interface_abis::IISubmission;
    let _ = songbird::interface_abis::IIVoterRegistrationTrigger;
    let _ = songbird::interface_abis::IIClaimSetupManager;
    let _ = songbird::interface_abis::IIRelay;
    let _ = songbird::interface_abis::IIRewardEpochSwitchoverTrigger;
    let _ = songbird::interface_abis::IIFlareSystemsCalculator;
    let _ = songbird::interface_abis::IINodePossessionVerifier;
    let _ = songbird::interface_abis::IIFlareSystemsManager;
    let _ = songbird::interface_abis::IICleanable;
    let _ = songbird::interface_abis::IIVPToken;
    let _ = songbird::interface_abis::IIVPContract;
    let _ = songbird::interface_abis::IIGovernanceVotePower;
    let _ = songbird::interface_abis::IIRNatAccount;
    let _ = songbird::interface_abis::IIRNat;
    let _ = songbird::interface_abis::IIFtsoFeedPublisher;
    let _ = songbird::interface_abis::IIFtso;
    let _ = songbird::interface_abis::IIPollingFoundation;
    let _ = songbird::interface_abis::IIGovernorProposer;
    let _ = songbird::interface_abis::Bn256;
    let _ = songbird::interface_abis::IIFastUpdater;
    let _ = songbird::interface_abis::IIFastUpdateIncentiveManager;
}

#[test]
fn interface_name_to_abi() {
    let _ = interface_to_abi(String::from("IWNat"), Songbird).expect("Interface IWNat not found");
    let _ = interface_to_abi(String::from("IDelegationAccount"), Songbird).expect("Interface IDelegationAccount not found");
    let _ = interface_to_abi(String::from("ICChainStake"), Songbird).expect("Interface ICChainStake not found");
    let _ = interface_to_abi(String::from("IPriceSubmitter"), Songbird).expect("Interface IPriceSubmitter not found");
    let _ = interface_to_abi(String::from("IFastUpdateIncentiveManager"), Songbird).expect("Interface IFastUpdateIncentiveManager not found");
    let _ = interface_to_abi(String::from("IFtsoFeedIdConverter"), Songbird).expect("Interface IFtsoFeedIdConverter not found");
    let _ = interface_to_abi(String::from("ICChainVotePower"), Songbird).expect("Interface ICChainVotePower not found");
    let _ = interface_to_abi(String::from("IRNatAccount"), Songbird).expect("Interface IRNatAccount not found");
    let _ = interface_to_abi(String::from("IEVMTransactionVerification"), Songbird).expect("Interface IEVMTransactionVerification not found");
    let _ = interface_to_abi(String::from("IFlareContractRegistry"), Songbird).expect("Interface IFlareContractRegistry not found");
    let _ = interface_to_abi(String::from("IGovernanceSettings"), Songbird).expect("Interface IGovernanceSettings not found");
    let _ = interface_to_abi(String::from("IRandomProvider"), Songbird).expect("Interface IRandomProvider not found");
    let _ = interface_to_abi(String::from("IValidatorRewardOffersManager"), Songbird).expect("Interface IValidatorRewardOffersManager not found");
    let _ = interface_to_abi(String::from("EVMTransaction"), Songbird).expect("Interface EVMTransaction not found");
    let _ = interface_to_abi(String::from("IPaymentVerification"), Songbird).expect("Interface IPaymentVerification not found");
    let _ = interface_to_abi(String::from("IVPToken"), Songbird).expect("Interface IVPToken not found");
    let _ = interface_to_abi(String::from("IFtsoInflationConfigurations"), Songbird).expect("Interface IFtsoInflationConfigurations not found");
    let _ = interface_to_abi(String::from("IGovernanceVotePower"), Songbird).expect("Interface IGovernanceVotePower not found");
    let _ = interface_to_abi(String::from("ISubmission"), Songbird).expect("Interface ISubmission not found");
    let _ = interface_to_abi(String::from("IBalanceDecreasingTransactionVerification"), Songbird).expect("Interface IBalanceDecreasingTransactionVerification not found");
    let _ = interface_to_abi(String::from("IFtsoManager"), Songbird).expect("Interface IFtsoManager not found");
    let _ = interface_to_abi(String::from("IEntityManager"), Songbird).expect("Interface IEntityManager not found");
    let _ = interface_to_abi(String::from("IPollingFtso"), Songbird).expect("Interface IPollingFtso not found");
    let _ = interface_to_abi(String::from("IFlareSystemsCalculator"), Songbird).expect("Interface IFlareSystemsCalculator not found");
    let _ = interface_to_abi(String::from("IFtsoFeedDecimals"), Songbird).expect("Interface IFtsoFeedDecimals not found");
    let _ = interface_to_abi(String::from("IReferencedPaymentNonexistenceVerification"), Songbird).expect("Interface IReferencedPaymentNonexistenceVerification not found");
    let _ = interface_to_abi(String::from("FlareContractsRegistryLibrary"), Songbird).expect("Interface FlareContractsRegistryLibrary not found");
    let _ = interface_to_abi(String::from("BalanceDecreasingTransaction"), Songbird).expect("Interface BalanceDecreasingTransaction not found");
    let _ = interface_to_abi(String::from("IIncreaseManager"), Songbird).expect("Interface IIncreaseManager not found");
    let _ = interface_to_abi(String::from("IMerkleRootStorage"), Songbird).expect("Interface IMerkleRootStorage not found");
    let _ = interface_to_abi(String::from("IStateConnector"), Songbird).expect("Interface IStateConnector not found");
    let _ = interface_to_abi(String::from("IFtsoRewardOffersManager"), Songbird).expect("Interface IFtsoRewardOffersManager not found");
    let _ = interface_to_abi(String::from("IClaimSetupManager"), Songbird).expect("Interface IClaimSetupManager not found");
    let _ = interface_to_abi(String::from("IConfirmedBlockHeightExistsVerification"), Songbird).expect("Interface IConfirmedBlockHeightExistsVerification not found");
    let _ = interface_to_abi(String::from("IAddressValidityVerification"), Songbird).expect("Interface IAddressValidityVerification not found");
    let _ = interface_to_abi(String::from("ReferencedPaymentNonexistence"), Songbird).expect("Interface ReferencedPaymentNonexistence not found");
    let _ = interface_to_abi(String::from("IFastUpdatesConfiguration"), Songbird).expect("Interface IFastUpdatesConfiguration not found");
    let _ = interface_to_abi(String::from("IGenericRewardManager"), Songbird).expect("Interface IGenericRewardManager not found");
    let _ = interface_to_abi(String::from("IFlareSystemsManager"), Songbird).expect("Interface IFlareSystemsManager not found");
    let _ = interface_to_abi(String::from("IFlareAssetRegistry"), Songbird).expect("Interface IFlareAssetRegistry not found");
    let _ = interface_to_abi(String::from("IGovernor"), Songbird).expect("Interface IGovernor not found");
    let _ = interface_to_abi(String::from("IFtsoRewardManager"), Songbird).expect("Interface IFtsoRewardManager not found");
    let _ = interface_to_abi(String::from("IVoterRegistry"), Songbird).expect("Interface IVoterRegistry not found");
    let _ = interface_to_abi(String::from("AddressValidity"), Songbird).expect("Interface AddressValidity not found");
    let _ = interface_to_abi(String::from("IWNatDelegationFee"), Songbird).expect("Interface IWNatDelegationFee not found");
    let _ = interface_to_abi(String::from("Payment"), Songbird).expect("Interface Payment not found");
    let _ = interface_to_abi(String::from("IVoterWhitelister"), Songbird).expect("Interface IVoterWhitelister not found");
    let _ = interface_to_abi(String::from("IFtsoFeedPublisher"), Songbird).expect("Interface IFtsoFeedPublisher not found");
    let _ = interface_to_abi(String::from("IRelay"), Songbird).expect("Interface IRelay not found");
    let _ = interface_to_abi(String::from("IRewardManager"), Songbird).expect("Interface IRewardManager not found");
    let _ = interface_to_abi(String::from("IFastUpdater"), Songbird).expect("Interface IFastUpdater not found");
    let _ = interface_to_abi(String::from("IRNat"), Songbird).expect("Interface IRNat not found");
    let _ = interface_to_abi(String::from("IVPContractEvents"), Songbird).expect("Interface IVPContractEvents not found");
    let _ = interface_to_abi(String::from("IFtso"), Songbird).expect("Interface IFtso not found");
    let _ = interface_to_abi(String::from("ConfirmedBlockHeightExists"), Songbird).expect("Interface ConfirmedBlockHeightExists not found");
    let _ = interface_to_abi(String::from("IFtsoRegistry"), Songbird).expect("Interface IFtsoRegistry not found");
    let _ = interface_to_abi(String::from("IFtsoGenesis"), Songbird).expect("Interface IFtsoGenesis not found");
    let _ = interface_to_abi(String::from("IFlareDaemonize"), Songbird).expect("Interface IFlareDaemonize not found");
    let _ = interface_to_abi(String::from("IFtsoRegistryGenesis"), Songbird).expect("Interface IFtsoRegistryGenesis not found");
    let _ = interface_to_abi(String::from("IFtsoManagerGenesis"), Songbird).expect("Interface IFtsoManagerGenesis not found");
    let _ = interface_to_abi(String::from("IIEntityManager"), Songbird).expect("Interface IIEntityManager not found");
    let _ = interface_to_abi(String::from("IICleanupBlockNumberManager"), Songbird).expect("Interface IICleanupBlockNumberManager not found");
    let _ = interface_to_abi(String::from("IIRewardManager"), Songbird).expect("Interface IIRewardManager not found");
    let _ = interface_to_abi(String::from("IIVoterRegistry"), Songbird).expect("Interface IIVoterRegistry not found");
    let _ = interface_to_abi(String::from("IIPublicKeyVerifier"), Songbird).expect("Interface IIPublicKeyVerifier not found");
    let _ = interface_to_abi(String::from("IISubmission"), Songbird).expect("Interface IISubmission not found");
    let _ = interface_to_abi(String::from("IIVoterRegistrationTrigger"), Songbird).expect("Interface IIVoterRegistrationTrigger not found");
    let _ = interface_to_abi(String::from("IIClaimSetupManager"), Songbird).expect("Interface IIClaimSetupManager not found");
    let _ = interface_to_abi(String::from("IIRelay"), Songbird).expect("Interface IIRelay not found");
    let _ = interface_to_abi(String::from("IIRewardEpochSwitchoverTrigger"), Songbird).expect("Interface IIRewardEpochSwitchoverTrigger not found");
    let _ = interface_to_abi(String::from("IIFlareSystemsCalculator"), Songbird).expect("Interface IIFlareSystemsCalculator not found");
    let _ = interface_to_abi(String::from("IINodePossessionVerifier"), Songbird).expect("Interface IINodePossessionVerifier not found");
    let _ = interface_to_abi(String::from("IIFlareSystemsManager"), Songbird).expect("Interface IIFlareSystemsManager not found");
    let _ = interface_to_abi(String::from("IICleanable"), Songbird).expect("Interface IICleanable not found");
    let _ = interface_to_abi(String::from("IIVPToken"), Songbird).expect("Interface IIVPToken not found");
    let _ = interface_to_abi(String::from("IIVPContract"), Songbird).expect("Interface IIVPContract not found");
    let _ = interface_to_abi(String::from("IIGovernanceVotePower"), Songbird).expect("Interface IIGovernanceVotePower not found");
    let _ = interface_to_abi(String::from("IIRNatAccount"), Songbird).expect("Interface IIRNatAccount not found");
    let _ = interface_to_abi(String::from("IIRNat"), Songbird).expect("Interface IIRNat not found");
    let _ = interface_to_abi(String::from("IIFtsoFeedPublisher"), Songbird).expect("Interface IIFtsoFeedPublisher not found");
    let _ = interface_to_abi(String::from("IIFtso"), Songbird).expect("Interface IIFtso not found");
    let _ = interface_to_abi(String::from("IIPollingFoundation"), Songbird).expect("Interface IIPollingFoundation not found");
    let _ = interface_to_abi(String::from("IIGovernorProposer"), Songbird).expect("Interface IIGovernorProposer not found");
    let _ = interface_to_abi(String::from("Bn256"), Songbird).expect("Interface Bn256 not found");
    let _ = interface_to_abi(String::from("IIFastUpdater"), Songbird).expect("Interface IIFastUpdater not found");
    let _ = interface_to_abi(String::from("IIFastUpdateIncentiveManager"), Songbird).expect("Interface IIFastUpdateIncentiveManager not found");
}

#[test]
fn abis_for_contracts() {
    let _ = songbird::products::WNat;
    let _ = songbird::products::PriceSubmitter;
    let _ = songbird::products::FastUpdateIncentiveManager;
    let _ = songbird::products::FtsoFeedIdConverter;
    let _ = songbird::products::FlareContractRegistry;
    let _ = songbird::products::GovernanceSettings;
    let _ = songbird::products::FtsoInflationConfigurations;
    let _ = songbird::products::GovernanceVotePower;
    let _ = songbird::products::Submission;
    let _ = songbird::products::FtsoManager;
    let _ = songbird::products::EntityManager;
    let _ = songbird::products::PollingFtso;
    let _ = songbird::products::FlareSystemsCalculator;
    let _ = songbird::products::FtsoFeedDecimals;
    let _ = songbird::products::StateConnector;
    let _ = songbird::products::FtsoRewardOffersManager;
    let _ = songbird::products::ClaimSetupManager;
    let _ = songbird::products::FastUpdatesConfiguration;
    let _ = songbird::products::FlareSystemsManager;
    let _ = songbird::products::FlareAssetRegistry;
    let _ = songbird::products::FtsoRewardManager;
    let _ = songbird::products::VoterRegistry;
    let _ = songbird::products::WNatDelegationFee;
    let _ = songbird::products::VoterWhitelister;
    let _ = songbird::products::FtsoFeedPublisher;
    let _ = songbird::products::Relay;
    let _ = songbird::products::RewardManager;
    let _ = songbird::products::FastUpdater;
    let _ = songbird::products::FtsoRegistry;
}

#[test]
fn contract_name_to_abi() {
    let _ = name_to_abi(String::from("WNat"), Songbird).expect("Contract WNat not found");
    let _ = name_to_abi(String::from("PriceSubmitter"), Songbird).expect("Contract PriceSubmitter not found");
    let _ = name_to_abi(String::from("FastUpdateIncentiveManager"), Songbird).expect("Contract FastUpdateIncentiveManager not found");
    let _ = name_to_abi(String::from("FtsoFeedIdConverter"), Songbird).expect("Contract FtsoFeedIdConverter not found");
    let _ = name_to_abi(String::from("FlareContractRegistry"), Songbird).expect("Contract FlareContractRegistry not found");
    let _ = name_to_abi(String::from("GovernanceSettings"), Songbird).expect("Contract GovernanceSettings not found");
    let _ = name_to_abi(String::from("FtsoInflationConfigurations"), Songbird).expect("Contract FtsoInflationConfigurations not found");
    let _ = name_to_abi(String::from("GovernanceVotePower"), Songbird).expect("Contract GovernanceVotePower not found");
    let _ = name_to_abi(String::from("Submission"), Songbird).expect("Contract Submission not found");
    let _ = name_to_abi(String::from("FtsoManager"), Songbird).expect("Contract FtsoManager not found");
    let _ = name_to_abi(String::from("EntityManager"), Songbird).expect("Contract EntityManager not found");
    let _ = name_to_abi(String::from("PollingFtso"), Songbird).expect("Contract PollingFtso not found");
    let _ = name_to_abi(String::from("FlareSystemsCalculator"), Songbird).expect("Contract FlareSystemsCalculator not found");
    let _ = name_to_abi(String::from("FtsoFeedDecimals"), Songbird).expect("Contract FtsoFeedDecimals not found");
    let _ = name_to_abi(String::from("StateConnector"), Songbird).expect("Contract StateConnector not found");
    let _ = name_to_abi(String::from("FtsoRewardOffersManager"), Songbird).expect("Contract FtsoRewardOffersManager not found");
    let _ = name_to_abi(String::from("ClaimSetupManager"), Songbird).expect("Contract ClaimSetupManager not found");
    let _ = name_to_abi(String::from("FastUpdatesConfiguration"), Songbird).expect("Contract FastUpdatesConfiguration not found");
    let _ = name_to_abi(String::from("FlareSystemsManager"), Songbird).expect("Contract FlareSystemsManager not found");
    let _ = name_to_abi(String::from("FlareAssetRegistry"), Songbird).expect("Contract FlareAssetRegistry not found");
    let _ = name_to_abi(String::from("FtsoRewardManager"), Songbird).expect("Contract FtsoRewardManager not found");
    let _ = name_to_abi(String::from("VoterRegistry"), Songbird).expect("Contract VoterRegistry not found");
    let _ = name_to_abi(String::from("WNatDelegationFee"), Songbird).expect("Contract WNatDelegationFee not found");
    let _ = name_to_abi(String::from("VoterWhitelister"), Songbird).expect("Contract VoterWhitelister not found");
    let _ = name_to_abi(String::from("FtsoFeedPublisher"), Songbird).expect("Contract FtsoFeedPublisher not found");
    let _ = name_to_abi(String::from("Relay"), Songbird).expect("Contract Relay not found");
    let _ = name_to_abi(String::from("RewardManager"), Songbird).expect("Contract RewardManager not found");
    let _ = name_to_abi(String::from("FastUpdater"), Songbird).expect("Contract FastUpdater not found");
    let _ = name_to_abi(String::from("FtsoRegistry"), Songbird).expect("Contract FtsoRegistry not found");
}

#[test]
fn toplevel_match() {
    assert_eq!(
        interface_to_abi(String::from("IWNat"), Songbird),
        name_to_abi(String::from("WNat"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IPriceSubmitter"), Songbird),
        name_to_abi(String::from("PriceSubmitter"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFastUpdateIncentiveManager"), Songbird),
        name_to_abi(String::from("FastUpdateIncentiveManager"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoFeedIdConverter"), Songbird),
        name_to_abi(String::from("FtsoFeedIdConverter"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFlareContractRegistry"), Songbird),
        name_to_abi(String::from("FlareContractRegistry"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IGovernanceSettings"), Songbird),
        name_to_abi(String::from("GovernanceSettings"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoInflationConfigurations"), Songbird),
        name_to_abi(String::from("FtsoInflationConfigurations"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IGovernanceVotePower"), Songbird),
        name_to_abi(String::from("GovernanceVotePower"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("ISubmission"), Songbird),
        name_to_abi(String::from("Submission"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoManager"), Songbird),
        name_to_abi(String::from("FtsoManager"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IEntityManager"), Songbird),
        name_to_abi(String::from("EntityManager"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IPollingFtso"), Songbird),
        name_to_abi(String::from("PollingFtso"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFlareSystemsCalculator"), Songbird),
        name_to_abi(String::from("FlareSystemsCalculator"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoFeedDecimals"), Songbird),
        name_to_abi(String::from("FtsoFeedDecimals"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IStateConnector"), Songbird),
        name_to_abi(String::from("StateConnector"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoRewardOffersManager"), Songbird),
        name_to_abi(String::from("FtsoRewardOffersManager"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IClaimSetupManager"), Songbird),
        name_to_abi(String::from("ClaimSetupManager"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFastUpdatesConfiguration"), Songbird),
        name_to_abi(String::from("FastUpdatesConfiguration"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFlareSystemsManager"), Songbird),
        name_to_abi(String::from("FlareSystemsManager"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFlareAssetRegistry"), Songbird),
        name_to_abi(String::from("FlareAssetRegistry"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoRewardManager"), Songbird),
        name_to_abi(String::from("FtsoRewardManager"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IVoterRegistry"), Songbird),
        name_to_abi(String::from("VoterRegistry"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IWNatDelegationFee"), Songbird),
        name_to_abi(String::from("WNatDelegationFee"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IVoterWhitelister"), Songbird),
        name_to_abi(String::from("VoterWhitelister"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoFeedPublisher"), Songbird),
        name_to_abi(String::from("FtsoFeedPublisher"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IRelay"), Songbird),
        name_to_abi(String::from("Relay"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IRewardManager"), Songbird),
        name_to_abi(String::from("RewardManager"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFastUpdater"), Songbird),
        name_to_abi(String::from("FastUpdater"), Songbird)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoRegistry"), Songbird),
        name_to_abi(String::from("FtsoRegistry"), Songbird)
    );
}

// This file was generated automatically. Do not edit manually.
