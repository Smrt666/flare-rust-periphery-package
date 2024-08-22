// This file was generated automatically. Do not edit manually.
use flare_rust_periphery_package::{coston, interface_to_abi, name_to_abi, Chain::Coston};

#[test]
fn abis_exist_for_all_contracts() {
    let _ = coston::interface_abis::IWNat;
    let _ = coston::interface_abis::IDelegationAccount;
    let _ = coston::interface_abis::ICChainStake;
    let _ = coston::interface_abis::IPriceSubmitter;
    let _ = coston::interface_abis::IFastUpdateIncentiveManager;
    let _ = coston::interface_abis::IFtsoFeedIdConverter;
    let _ = coston::interface_abis::ICChainVotePower;
    let _ = coston::interface_abis::IRNatAccount;
    let _ = coston::interface_abis::IEVMTransactionVerification;
    let _ = coston::interface_abis::IFlareContractRegistry;
    let _ = coston::interface_abis::IGovernanceSettings;
    let _ = coston::interface_abis::IRandomProvider;
    let _ = coston::interface_abis::IValidatorRewardOffersManager;
    let _ = coston::interface_abis::EVMTransaction;
    let _ = coston::interface_abis::IPaymentVerification;
    let _ = coston::interface_abis::IVPToken;
    let _ = coston::interface_abis::IFtsoInflationConfigurations;
    let _ = coston::interface_abis::IGovernanceVotePower;
    let _ = coston::interface_abis::ISubmission;
    let _ = coston::interface_abis::IBalanceDecreasingTransactionVerification;
    let _ = coston::interface_abis::IFtsoManager;
    let _ = coston::interface_abis::IEntityManager;
    let _ = coston::interface_abis::IPollingFtso;
    let _ = coston::interface_abis::IFlareSystemsCalculator;
    let _ = coston::interface_abis::IFtsoFeedDecimals;
    let _ = coston::interface_abis::IReferencedPaymentNonexistenceVerification;
    let _ = coston::interface_abis::FlareContractsRegistryLibrary;
    let _ = coston::interface_abis::BalanceDecreasingTransaction;
    let _ = coston::interface_abis::IIncreaseManager;
    let _ = coston::interface_abis::IMerkleRootStorage;
    let _ = coston::interface_abis::IStateConnector;
    let _ = coston::interface_abis::IFtsoRewardOffersManager;
    let _ = coston::interface_abis::IClaimSetupManager;
    let _ = coston::interface_abis::IConfirmedBlockHeightExistsVerification;
    let _ = coston::interface_abis::IAddressValidityVerification;
    let _ = coston::interface_abis::ReferencedPaymentNonexistence;
    let _ = coston::interface_abis::IFastUpdatesConfiguration;
    let _ = coston::interface_abis::IGenericRewardManager;
    let _ = coston::interface_abis::IFlareSystemsManager;
    let _ = coston::interface_abis::IFlareAssetRegistry;
    let _ = coston::interface_abis::IGovernor;
    let _ = coston::interface_abis::IFtsoRewardManager;
    let _ = coston::interface_abis::IVoterRegistry;
    let _ = coston::interface_abis::AddressValidity;
    let _ = coston::interface_abis::IWNatDelegationFee;
    let _ = coston::interface_abis::Payment;
    let _ = coston::interface_abis::IVoterWhitelister;
    let _ = coston::interface_abis::IFtsoFeedPublisher;
    let _ = coston::interface_abis::IRelay;
    let _ = coston::interface_abis::IRewardManager;
    let _ = coston::interface_abis::IFastUpdater;
    let _ = coston::interface_abis::IRNat;
    let _ = coston::interface_abis::IVPContractEvents;
    let _ = coston::interface_abis::IFtso;
    let _ = coston::interface_abis::ConfirmedBlockHeightExists;
    let _ = coston::interface_abis::IFtsoRegistry;
    let _ = coston::interface_abis::IFtsoGenesis;
    let _ = coston::interface_abis::IFlareDaemonize;
    let _ = coston::interface_abis::IFtsoRegistryGenesis;
    let _ = coston::interface_abis::IFtsoManagerGenesis;
    let _ = coston::interface_abis::IIEntityManager;
    let _ = coston::interface_abis::IICleanupBlockNumberManager;
    let _ = coston::interface_abis::IIRewardManager;
    let _ = coston::interface_abis::IIVoterRegistry;
    let _ = coston::interface_abis::IIPublicKeyVerifier;
    let _ = coston::interface_abis::IISubmission;
    let _ = coston::interface_abis::IIVoterRegistrationTrigger;
    let _ = coston::interface_abis::IIClaimSetupManager;
    let _ = coston::interface_abis::IIRelay;
    let _ = coston::interface_abis::IIRewardEpochSwitchoverTrigger;
    let _ = coston::interface_abis::IIFlareSystemsCalculator;
    let _ = coston::interface_abis::IINodePossessionVerifier;
    let _ = coston::interface_abis::IIFlareSystemsManager;
    let _ = coston::interface_abis::IICleanable;
    let _ = coston::interface_abis::IIVPToken;
    let _ = coston::interface_abis::IIVPContract;
    let _ = coston::interface_abis::IIGovernanceVotePower;
    let _ = coston::interface_abis::IIRNatAccount;
    let _ = coston::interface_abis::IIRNat;
    let _ = coston::interface_abis::IIFtsoFeedPublisher;
    let _ = coston::interface_abis::IIFtso;
    let _ = coston::interface_abis::IIPollingFoundation;
    let _ = coston::interface_abis::IIGovernorProposer;
    let _ = coston::interface_abis::Bn256;
    let _ = coston::interface_abis::IIFastUpdater;
    let _ = coston::interface_abis::IIFastUpdateIncentiveManager;
}

#[test]
fn interface_name_to_abi() {
    let _ = interface_to_abi(String::from("IWNat"), Coston).expect("Interface IWNat not found");
    let _ = interface_to_abi(String::from("IDelegationAccount"), Coston).expect("Interface IDelegationAccount not found");
    let _ = interface_to_abi(String::from("ICChainStake"), Coston).expect("Interface ICChainStake not found");
    let _ = interface_to_abi(String::from("IPriceSubmitter"), Coston).expect("Interface IPriceSubmitter not found");
    let _ = interface_to_abi(String::from("IFastUpdateIncentiveManager"), Coston).expect("Interface IFastUpdateIncentiveManager not found");
    let _ = interface_to_abi(String::from("IFtsoFeedIdConverter"), Coston).expect("Interface IFtsoFeedIdConverter not found");
    let _ = interface_to_abi(String::from("ICChainVotePower"), Coston).expect("Interface ICChainVotePower not found");
    let _ = interface_to_abi(String::from("IRNatAccount"), Coston).expect("Interface IRNatAccount not found");
    let _ = interface_to_abi(String::from("IEVMTransactionVerification"), Coston).expect("Interface IEVMTransactionVerification not found");
    let _ = interface_to_abi(String::from("IFlareContractRegistry"), Coston).expect("Interface IFlareContractRegistry not found");
    let _ = interface_to_abi(String::from("IGovernanceSettings"), Coston).expect("Interface IGovernanceSettings not found");
    let _ = interface_to_abi(String::from("IRandomProvider"), Coston).expect("Interface IRandomProvider not found");
    let _ = interface_to_abi(String::from("IValidatorRewardOffersManager"), Coston).expect("Interface IValidatorRewardOffersManager not found");
    let _ = interface_to_abi(String::from("EVMTransaction"), Coston).expect("Interface EVMTransaction not found");
    let _ = interface_to_abi(String::from("IPaymentVerification"), Coston).expect("Interface IPaymentVerification not found");
    let _ = interface_to_abi(String::from("IVPToken"), Coston).expect("Interface IVPToken not found");
    let _ = interface_to_abi(String::from("IFtsoInflationConfigurations"), Coston).expect("Interface IFtsoInflationConfigurations not found");
    let _ = interface_to_abi(String::from("IGovernanceVotePower"), Coston).expect("Interface IGovernanceVotePower not found");
    let _ = interface_to_abi(String::from("ISubmission"), Coston).expect("Interface ISubmission not found");
    let _ = interface_to_abi(String::from("IBalanceDecreasingTransactionVerification"), Coston).expect("Interface IBalanceDecreasingTransactionVerification not found");
    let _ = interface_to_abi(String::from("IFtsoManager"), Coston).expect("Interface IFtsoManager not found");
    let _ = interface_to_abi(String::from("IEntityManager"), Coston).expect("Interface IEntityManager not found");
    let _ = interface_to_abi(String::from("IPollingFtso"), Coston).expect("Interface IPollingFtso not found");
    let _ = interface_to_abi(String::from("IFlareSystemsCalculator"), Coston).expect("Interface IFlareSystemsCalculator not found");
    let _ = interface_to_abi(String::from("IFtsoFeedDecimals"), Coston).expect("Interface IFtsoFeedDecimals not found");
    let _ = interface_to_abi(String::from("IReferencedPaymentNonexistenceVerification"), Coston).expect("Interface IReferencedPaymentNonexistenceVerification not found");
    let _ = interface_to_abi(String::from("FlareContractsRegistryLibrary"), Coston).expect("Interface FlareContractsRegistryLibrary not found");
    let _ = interface_to_abi(String::from("BalanceDecreasingTransaction"), Coston).expect("Interface BalanceDecreasingTransaction not found");
    let _ = interface_to_abi(String::from("IIncreaseManager"), Coston).expect("Interface IIncreaseManager not found");
    let _ = interface_to_abi(String::from("IMerkleRootStorage"), Coston).expect("Interface IMerkleRootStorage not found");
    let _ = interface_to_abi(String::from("IStateConnector"), Coston).expect("Interface IStateConnector not found");
    let _ = interface_to_abi(String::from("IFtsoRewardOffersManager"), Coston).expect("Interface IFtsoRewardOffersManager not found");
    let _ = interface_to_abi(String::from("IClaimSetupManager"), Coston).expect("Interface IClaimSetupManager not found");
    let _ = interface_to_abi(String::from("IConfirmedBlockHeightExistsVerification"), Coston).expect("Interface IConfirmedBlockHeightExistsVerification not found");
    let _ = interface_to_abi(String::from("IAddressValidityVerification"), Coston).expect("Interface IAddressValidityVerification not found");
    let _ = interface_to_abi(String::from("ReferencedPaymentNonexistence"), Coston).expect("Interface ReferencedPaymentNonexistence not found");
    let _ = interface_to_abi(String::from("IFastUpdatesConfiguration"), Coston).expect("Interface IFastUpdatesConfiguration not found");
    let _ = interface_to_abi(String::from("IGenericRewardManager"), Coston).expect("Interface IGenericRewardManager not found");
    let _ = interface_to_abi(String::from("IFlareSystemsManager"), Coston).expect("Interface IFlareSystemsManager not found");
    let _ = interface_to_abi(String::from("IFlareAssetRegistry"), Coston).expect("Interface IFlareAssetRegistry not found");
    let _ = interface_to_abi(String::from("IGovernor"), Coston).expect("Interface IGovernor not found");
    let _ = interface_to_abi(String::from("IFtsoRewardManager"), Coston).expect("Interface IFtsoRewardManager not found");
    let _ = interface_to_abi(String::from("IVoterRegistry"), Coston).expect("Interface IVoterRegistry not found");
    let _ = interface_to_abi(String::from("AddressValidity"), Coston).expect("Interface AddressValidity not found");
    let _ = interface_to_abi(String::from("IWNatDelegationFee"), Coston).expect("Interface IWNatDelegationFee not found");
    let _ = interface_to_abi(String::from("Payment"), Coston).expect("Interface Payment not found");
    let _ = interface_to_abi(String::from("IVoterWhitelister"), Coston).expect("Interface IVoterWhitelister not found");
    let _ = interface_to_abi(String::from("IFtsoFeedPublisher"), Coston).expect("Interface IFtsoFeedPublisher not found");
    let _ = interface_to_abi(String::from("IRelay"), Coston).expect("Interface IRelay not found");
    let _ = interface_to_abi(String::from("IRewardManager"), Coston).expect("Interface IRewardManager not found");
    let _ = interface_to_abi(String::from("IFastUpdater"), Coston).expect("Interface IFastUpdater not found");
    let _ = interface_to_abi(String::from("IRNat"), Coston).expect("Interface IRNat not found");
    let _ = interface_to_abi(String::from("IVPContractEvents"), Coston).expect("Interface IVPContractEvents not found");
    let _ = interface_to_abi(String::from("IFtso"), Coston).expect("Interface IFtso not found");
    let _ = interface_to_abi(String::from("ConfirmedBlockHeightExists"), Coston).expect("Interface ConfirmedBlockHeightExists not found");
    let _ = interface_to_abi(String::from("IFtsoRegistry"), Coston).expect("Interface IFtsoRegistry not found");
    let _ = interface_to_abi(String::from("IFtsoGenesis"), Coston).expect("Interface IFtsoGenesis not found");
    let _ = interface_to_abi(String::from("IFlareDaemonize"), Coston).expect("Interface IFlareDaemonize not found");
    let _ = interface_to_abi(String::from("IFtsoRegistryGenesis"), Coston).expect("Interface IFtsoRegistryGenesis not found");
    let _ = interface_to_abi(String::from("IFtsoManagerGenesis"), Coston).expect("Interface IFtsoManagerGenesis not found");
    let _ = interface_to_abi(String::from("IIEntityManager"), Coston).expect("Interface IIEntityManager not found");
    let _ = interface_to_abi(String::from("IICleanupBlockNumberManager"), Coston).expect("Interface IICleanupBlockNumberManager not found");
    let _ = interface_to_abi(String::from("IIRewardManager"), Coston).expect("Interface IIRewardManager not found");
    let _ = interface_to_abi(String::from("IIVoterRegistry"), Coston).expect("Interface IIVoterRegistry not found");
    let _ = interface_to_abi(String::from("IIPublicKeyVerifier"), Coston).expect("Interface IIPublicKeyVerifier not found");
    let _ = interface_to_abi(String::from("IISubmission"), Coston).expect("Interface IISubmission not found");
    let _ = interface_to_abi(String::from("IIVoterRegistrationTrigger"), Coston).expect("Interface IIVoterRegistrationTrigger not found");
    let _ = interface_to_abi(String::from("IIClaimSetupManager"), Coston).expect("Interface IIClaimSetupManager not found");
    let _ = interface_to_abi(String::from("IIRelay"), Coston).expect("Interface IIRelay not found");
    let _ = interface_to_abi(String::from("IIRewardEpochSwitchoverTrigger"), Coston).expect("Interface IIRewardEpochSwitchoverTrigger not found");
    let _ = interface_to_abi(String::from("IIFlareSystemsCalculator"), Coston).expect("Interface IIFlareSystemsCalculator not found");
    let _ = interface_to_abi(String::from("IINodePossessionVerifier"), Coston).expect("Interface IINodePossessionVerifier not found");
    let _ = interface_to_abi(String::from("IIFlareSystemsManager"), Coston).expect("Interface IIFlareSystemsManager not found");
    let _ = interface_to_abi(String::from("IICleanable"), Coston).expect("Interface IICleanable not found");
    let _ = interface_to_abi(String::from("IIVPToken"), Coston).expect("Interface IIVPToken not found");
    let _ = interface_to_abi(String::from("IIVPContract"), Coston).expect("Interface IIVPContract not found");
    let _ = interface_to_abi(String::from("IIGovernanceVotePower"), Coston).expect("Interface IIGovernanceVotePower not found");
    let _ = interface_to_abi(String::from("IIRNatAccount"), Coston).expect("Interface IIRNatAccount not found");
    let _ = interface_to_abi(String::from("IIRNat"), Coston).expect("Interface IIRNat not found");
    let _ = interface_to_abi(String::from("IIFtsoFeedPublisher"), Coston).expect("Interface IIFtsoFeedPublisher not found");
    let _ = interface_to_abi(String::from("IIFtso"), Coston).expect("Interface IIFtso not found");
    let _ = interface_to_abi(String::from("IIPollingFoundation"), Coston).expect("Interface IIPollingFoundation not found");
    let _ = interface_to_abi(String::from("IIGovernorProposer"), Coston).expect("Interface IIGovernorProposer not found");
    let _ = interface_to_abi(String::from("Bn256"), Coston).expect("Interface Bn256 not found");
    let _ = interface_to_abi(String::from("IIFastUpdater"), Coston).expect("Interface IIFastUpdater not found");
    let _ = interface_to_abi(String::from("IIFastUpdateIncentiveManager"), Coston).expect("Interface IIFastUpdateIncentiveManager not found");
}

#[test]
fn abis_for_contracts() {
    let _ = coston::products::WNat;
    let _ = coston::products::PriceSubmitter;
    let _ = coston::products::FastUpdateIncentiveManager;
    let _ = coston::products::FtsoFeedIdConverter;
    let _ = coston::products::FlareContractRegistry;
    let _ = coston::products::GovernanceSettings;
    let _ = coston::products::FtsoInflationConfigurations;
    let _ = coston::products::GovernanceVotePower;
    let _ = coston::products::Submission;
    let _ = coston::products::FtsoManager;
    let _ = coston::products::EntityManager;
    let _ = coston::products::PollingFtso;
    let _ = coston::products::FlareSystemsCalculator;
    let _ = coston::products::FtsoFeedDecimals;
    let _ = coston::products::StateConnector;
    let _ = coston::products::FtsoRewardOffersManager;
    let _ = coston::products::ClaimSetupManager;
    let _ = coston::products::FastUpdatesConfiguration;
    let _ = coston::products::FlareSystemsManager;
    let _ = coston::products::FlareAssetRegistry;
    let _ = coston::products::FtsoRewardManager;
    let _ = coston::products::VoterRegistry;
    let _ = coston::products::WNatDelegationFee;
    let _ = coston::products::VoterWhitelister;
    let _ = coston::products::FtsoFeedPublisher;
    let _ = coston::products::Relay;
    let _ = coston::products::RewardManager;
    let _ = coston::products::FastUpdater;
    let _ = coston::products::FtsoRegistry;
}

#[test]
fn contract_name_to_abi() {
    let _ = name_to_abi(String::from("WNat"), Coston).expect("Contract WNat not found");
    let _ = name_to_abi(String::from("PriceSubmitter"), Coston).expect("Contract PriceSubmitter not found");
    let _ = name_to_abi(String::from("FastUpdateIncentiveManager"), Coston).expect("Contract FastUpdateIncentiveManager not found");
    let _ = name_to_abi(String::from("FtsoFeedIdConverter"), Coston).expect("Contract FtsoFeedIdConverter not found");
    let _ = name_to_abi(String::from("FlareContractRegistry"), Coston).expect("Contract FlareContractRegistry not found");
    let _ = name_to_abi(String::from("GovernanceSettings"), Coston).expect("Contract GovernanceSettings not found");
    let _ = name_to_abi(String::from("FtsoInflationConfigurations"), Coston).expect("Contract FtsoInflationConfigurations not found");
    let _ = name_to_abi(String::from("GovernanceVotePower"), Coston).expect("Contract GovernanceVotePower not found");
    let _ = name_to_abi(String::from("Submission"), Coston).expect("Contract Submission not found");
    let _ = name_to_abi(String::from("FtsoManager"), Coston).expect("Contract FtsoManager not found");
    let _ = name_to_abi(String::from("EntityManager"), Coston).expect("Contract EntityManager not found");
    let _ = name_to_abi(String::from("PollingFtso"), Coston).expect("Contract PollingFtso not found");
    let _ = name_to_abi(String::from("FlareSystemsCalculator"), Coston).expect("Contract FlareSystemsCalculator not found");
    let _ = name_to_abi(String::from("FtsoFeedDecimals"), Coston).expect("Contract FtsoFeedDecimals not found");
    let _ = name_to_abi(String::from("StateConnector"), Coston).expect("Contract StateConnector not found");
    let _ = name_to_abi(String::from("FtsoRewardOffersManager"), Coston).expect("Contract FtsoRewardOffersManager not found");
    let _ = name_to_abi(String::from("ClaimSetupManager"), Coston).expect("Contract ClaimSetupManager not found");
    let _ = name_to_abi(String::from("FastUpdatesConfiguration"), Coston).expect("Contract FastUpdatesConfiguration not found");
    let _ = name_to_abi(String::from("FlareSystemsManager"), Coston).expect("Contract FlareSystemsManager not found");
    let _ = name_to_abi(String::from("FlareAssetRegistry"), Coston).expect("Contract FlareAssetRegistry not found");
    let _ = name_to_abi(String::from("FtsoRewardManager"), Coston).expect("Contract FtsoRewardManager not found");
    let _ = name_to_abi(String::from("VoterRegistry"), Coston).expect("Contract VoterRegistry not found");
    let _ = name_to_abi(String::from("WNatDelegationFee"), Coston).expect("Contract WNatDelegationFee not found");
    let _ = name_to_abi(String::from("VoterWhitelister"), Coston).expect("Contract VoterWhitelister not found");
    let _ = name_to_abi(String::from("FtsoFeedPublisher"), Coston).expect("Contract FtsoFeedPublisher not found");
    let _ = name_to_abi(String::from("Relay"), Coston).expect("Contract Relay not found");
    let _ = name_to_abi(String::from("RewardManager"), Coston).expect("Contract RewardManager not found");
    let _ = name_to_abi(String::from("FastUpdater"), Coston).expect("Contract FastUpdater not found");
    let _ = name_to_abi(String::from("FtsoRegistry"), Coston).expect("Contract FtsoRegistry not found");
}

#[test]
fn toplevel_match() {
    assert_eq!(
        interface_to_abi(String::from("IWNat"), Coston),
        name_to_abi(String::from("WNat"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IPriceSubmitter"), Coston),
        name_to_abi(String::from("PriceSubmitter"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFastUpdateIncentiveManager"), Coston),
        name_to_abi(String::from("FastUpdateIncentiveManager"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoFeedIdConverter"), Coston),
        name_to_abi(String::from("FtsoFeedIdConverter"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFlareContractRegistry"), Coston),
        name_to_abi(String::from("FlareContractRegistry"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IGovernanceSettings"), Coston),
        name_to_abi(String::from("GovernanceSettings"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoInflationConfigurations"), Coston),
        name_to_abi(String::from("FtsoInflationConfigurations"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IGovernanceVotePower"), Coston),
        name_to_abi(String::from("GovernanceVotePower"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("ISubmission"), Coston),
        name_to_abi(String::from("Submission"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoManager"), Coston),
        name_to_abi(String::from("FtsoManager"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IEntityManager"), Coston),
        name_to_abi(String::from("EntityManager"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IPollingFtso"), Coston),
        name_to_abi(String::from("PollingFtso"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFlareSystemsCalculator"), Coston),
        name_to_abi(String::from("FlareSystemsCalculator"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoFeedDecimals"), Coston),
        name_to_abi(String::from("FtsoFeedDecimals"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IStateConnector"), Coston),
        name_to_abi(String::from("StateConnector"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoRewardOffersManager"), Coston),
        name_to_abi(String::from("FtsoRewardOffersManager"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IClaimSetupManager"), Coston),
        name_to_abi(String::from("ClaimSetupManager"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFastUpdatesConfiguration"), Coston),
        name_to_abi(String::from("FastUpdatesConfiguration"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFlareSystemsManager"), Coston),
        name_to_abi(String::from("FlareSystemsManager"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFlareAssetRegistry"), Coston),
        name_to_abi(String::from("FlareAssetRegistry"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoRewardManager"), Coston),
        name_to_abi(String::from("FtsoRewardManager"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IVoterRegistry"), Coston),
        name_to_abi(String::from("VoterRegistry"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IWNatDelegationFee"), Coston),
        name_to_abi(String::from("WNatDelegationFee"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IVoterWhitelister"), Coston),
        name_to_abi(String::from("VoterWhitelister"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoFeedPublisher"), Coston),
        name_to_abi(String::from("FtsoFeedPublisher"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IRelay"), Coston),
        name_to_abi(String::from("Relay"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IRewardManager"), Coston),
        name_to_abi(String::from("RewardManager"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFastUpdater"), Coston),
        name_to_abi(String::from("FastUpdater"), Coston)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoRegistry"), Coston),
        name_to_abi(String::from("FtsoRegistry"), Coston)
    );
}

// This file was generated automatically. Do not edit manually.
