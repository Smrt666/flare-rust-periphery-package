// This file was generated automatically. Do not edit manually.
use flare_rust_periphery_package::{flare, interface_to_abi, name_to_abi, Chain::Flare};

#[test]
fn abis_exist_for_all_contracts() {
    let _ = flare::interface_abis::IWNat;
    let _ = flare::interface_abis::IDelegationAccount;
    let _ = flare::interface_abis::ICChainStake;
    let _ = flare::interface_abis::IPriceSubmitter;
    let _ = flare::interface_abis::IFastUpdateIncentiveManager;
    let _ = flare::interface_abis::IFtsoFeedIdConverter;
    let _ = flare::interface_abis::ICChainVotePower;
    let _ = flare::interface_abis::IPChainStakeMirror;
    let _ = flare::interface_abis::IRNatAccount;
    let _ = flare::interface_abis::IFlareContractRegistry;
    let _ = flare::interface_abis::IGovernanceSettings;
    let _ = flare::interface_abis::IRandomProvider;
    let _ = flare::interface_abis::IValidatorRewardOffersManager;
    let _ = flare::interface_abis::IVPToken;
    let _ = flare::interface_abis::IFtsoInflationConfigurations;
    let _ = flare::interface_abis::IGovernanceVotePower;
    let _ = flare::interface_abis::ISubmission;
    let _ = flare::interface_abis::IFtsoManager;
    let _ = flare::interface_abis::IEntityManager;
    let _ = flare::interface_abis::IPollingFtso;
    let _ = flare::interface_abis::IFlareSystemsCalculator;
    let _ = flare::interface_abis::IFtsoFeedDecimals;
    let _ = flare::interface_abis::FlareContractsRegistryLibrary;
    let _ = flare::interface_abis::IIncreaseManager;
    let _ = flare::interface_abis::IPChainStakeMirrorVerifier;
    let _ = flare::interface_abis::IPChainVotePower;
    let _ = flare::interface_abis::IFtsoRewardOffersManager;
    let _ = flare::interface_abis::IClaimSetupManager;
    let _ = flare::interface_abis::IDistributionToDelegators;
    let _ = flare::interface_abis::IFastUpdatesConfiguration;
    let _ = flare::interface_abis::IGenericRewardManager;
    let _ = flare::interface_abis::IFlareSystemsManager;
    let _ = flare::interface_abis::IFlareAssetRegistry;
    let _ = flare::interface_abis::IGovernor;
    let _ = flare::interface_abis::IFtsoRewardManager;
    let _ = flare::interface_abis::IVoterRegistry;
    let _ = flare::interface_abis::IWNatDelegationFee;
    let _ = flare::interface_abis::IVoterWhitelister;
    let _ = flare::interface_abis::IFtsoFeedPublisher;
    let _ = flare::interface_abis::IRelay;
    let _ = flare::interface_abis::IRewardManager;
    let _ = flare::interface_abis::IAddressBinder;
    let _ = flare::interface_abis::IFastUpdater;
    let _ = flare::interface_abis::IPChainStakeMirrorMultiSigVoting;
    let _ = flare::interface_abis::IRNat;
    let _ = flare::interface_abis::IVPContractEvents;
    let _ = flare::interface_abis::IFtso;
    let _ = flare::interface_abis::IValidatorRegistry;
    let _ = flare::interface_abis::IFtsoRegistry;
    let _ = flare::interface_abis::IFtsoGenesis;
    let _ = flare::interface_abis::IFlareDaemonize;
    let _ = flare::interface_abis::IFtsoRegistryGenesis;
    let _ = flare::interface_abis::IFtsoManagerGenesis;
    let _ = flare::interface_abis::IIEntityManager;
    let _ = flare::interface_abis::IICleanupBlockNumberManager;
    let _ = flare::interface_abis::IIRewardManager;
    let _ = flare::interface_abis::IIVoterRegistry;
    let _ = flare::interface_abis::IIPublicKeyVerifier;
    let _ = flare::interface_abis::IISubmission;
    let _ = flare::interface_abis::IIVoterRegistrationTrigger;
    let _ = flare::interface_abis::IIClaimSetupManager;
    let _ = flare::interface_abis::IIRelay;
    let _ = flare::interface_abis::IIRewardEpochSwitchoverTrigger;
    let _ = flare::interface_abis::IIFlareSystemsCalculator;
    let _ = flare::interface_abis::IINodePossessionVerifier;
    let _ = flare::interface_abis::IIFlareSystemsManager;
    let _ = flare::interface_abis::IICleanable;
    let _ = flare::interface_abis::IIVPToken;
    let _ = flare::interface_abis::IIVPContract;
    let _ = flare::interface_abis::IIGovernanceVotePower;
    let _ = flare::interface_abis::IIRNatAccount;
    let _ = flare::interface_abis::IIRNat;
    let _ = flare::interface_abis::IIFtsoFeedPublisher;
    let _ = flare::interface_abis::IIFtso;
    let _ = flare::interface_abis::IIPollingFoundation;
    let _ = flare::interface_abis::IIGovernorProposer;
    let _ = flare::interface_abis::Bn256;
    let _ = flare::interface_abis::IIFastUpdater;
    let _ = flare::interface_abis::IIFastUpdateIncentiveManager;
}

#[test]
fn interface_name_to_abi() {
    let _ = interface_to_abi(String::from("IWNat"), Flare).expect("Interface IWNat not found");
    let _ = interface_to_abi(String::from("IDelegationAccount"), Flare).expect("Interface IDelegationAccount not found");
    let _ = interface_to_abi(String::from("ICChainStake"), Flare).expect("Interface ICChainStake not found");
    let _ = interface_to_abi(String::from("IPriceSubmitter"), Flare).expect("Interface IPriceSubmitter not found");
    let _ = interface_to_abi(String::from("IFastUpdateIncentiveManager"), Flare).expect("Interface IFastUpdateIncentiveManager not found");
    let _ = interface_to_abi(String::from("IFtsoFeedIdConverter"), Flare).expect("Interface IFtsoFeedIdConverter not found");
    let _ = interface_to_abi(String::from("ICChainVotePower"), Flare).expect("Interface ICChainVotePower not found");
    let _ = interface_to_abi(String::from("IPChainStakeMirror"), Flare).expect("Interface IPChainStakeMirror not found");
    let _ = interface_to_abi(String::from("IRNatAccount"), Flare).expect("Interface IRNatAccount not found");
    let _ = interface_to_abi(String::from("IFlareContractRegistry"), Flare).expect("Interface IFlareContractRegistry not found");
    let _ = interface_to_abi(String::from("IGovernanceSettings"), Flare).expect("Interface IGovernanceSettings not found");
    let _ = interface_to_abi(String::from("IRandomProvider"), Flare).expect("Interface IRandomProvider not found");
    let _ = interface_to_abi(String::from("IValidatorRewardOffersManager"), Flare).expect("Interface IValidatorRewardOffersManager not found");
    let _ = interface_to_abi(String::from("IVPToken"), Flare).expect("Interface IVPToken not found");
    let _ = interface_to_abi(String::from("IFtsoInflationConfigurations"), Flare).expect("Interface IFtsoInflationConfigurations not found");
    let _ = interface_to_abi(String::from("IGovernanceVotePower"), Flare).expect("Interface IGovernanceVotePower not found");
    let _ = interface_to_abi(String::from("ISubmission"), Flare).expect("Interface ISubmission not found");
    let _ = interface_to_abi(String::from("IFtsoManager"), Flare).expect("Interface IFtsoManager not found");
    let _ = interface_to_abi(String::from("IEntityManager"), Flare).expect("Interface IEntityManager not found");
    let _ = interface_to_abi(String::from("IPollingFtso"), Flare).expect("Interface IPollingFtso not found");
    let _ = interface_to_abi(String::from("IFlareSystemsCalculator"), Flare).expect("Interface IFlareSystemsCalculator not found");
    let _ = interface_to_abi(String::from("IFtsoFeedDecimals"), Flare).expect("Interface IFtsoFeedDecimals not found");
    let _ = interface_to_abi(String::from("FlareContractsRegistryLibrary"), Flare).expect("Interface FlareContractsRegistryLibrary not found");
    let _ = interface_to_abi(String::from("IIncreaseManager"), Flare).expect("Interface IIncreaseManager not found");
    let _ = interface_to_abi(String::from("IPChainStakeMirrorVerifier"), Flare).expect("Interface IPChainStakeMirrorVerifier not found");
    let _ = interface_to_abi(String::from("IPChainVotePower"), Flare).expect("Interface IPChainVotePower not found");
    let _ = interface_to_abi(String::from("IFtsoRewardOffersManager"), Flare).expect("Interface IFtsoRewardOffersManager not found");
    let _ = interface_to_abi(String::from("IClaimSetupManager"), Flare).expect("Interface IClaimSetupManager not found");
    let _ = interface_to_abi(String::from("IDistributionToDelegators"), Flare).expect("Interface IDistributionToDelegators not found");
    let _ = interface_to_abi(String::from("IFastUpdatesConfiguration"), Flare).expect("Interface IFastUpdatesConfiguration not found");
    let _ = interface_to_abi(String::from("IGenericRewardManager"), Flare).expect("Interface IGenericRewardManager not found");
    let _ = interface_to_abi(String::from("IFlareSystemsManager"), Flare).expect("Interface IFlareSystemsManager not found");
    let _ = interface_to_abi(String::from("IFlareAssetRegistry"), Flare).expect("Interface IFlareAssetRegistry not found");
    let _ = interface_to_abi(String::from("IGovernor"), Flare).expect("Interface IGovernor not found");
    let _ = interface_to_abi(String::from("IFtsoRewardManager"), Flare).expect("Interface IFtsoRewardManager not found");
    let _ = interface_to_abi(String::from("IVoterRegistry"), Flare).expect("Interface IVoterRegistry not found");
    let _ = interface_to_abi(String::from("IWNatDelegationFee"), Flare).expect("Interface IWNatDelegationFee not found");
    let _ = interface_to_abi(String::from("IVoterWhitelister"), Flare).expect("Interface IVoterWhitelister not found");
    let _ = interface_to_abi(String::from("IFtsoFeedPublisher"), Flare).expect("Interface IFtsoFeedPublisher not found");
    let _ = interface_to_abi(String::from("IRelay"), Flare).expect("Interface IRelay not found");
    let _ = interface_to_abi(String::from("IRewardManager"), Flare).expect("Interface IRewardManager not found");
    let _ = interface_to_abi(String::from("IAddressBinder"), Flare).expect("Interface IAddressBinder not found");
    let _ = interface_to_abi(String::from("IFastUpdater"), Flare).expect("Interface IFastUpdater not found");
    let _ = interface_to_abi(String::from("IPChainStakeMirrorMultiSigVoting"), Flare).expect("Interface IPChainStakeMirrorMultiSigVoting not found");
    let _ = interface_to_abi(String::from("IRNat"), Flare).expect("Interface IRNat not found");
    let _ = interface_to_abi(String::from("IVPContractEvents"), Flare).expect("Interface IVPContractEvents not found");
    let _ = interface_to_abi(String::from("IFtso"), Flare).expect("Interface IFtso not found");
    let _ = interface_to_abi(String::from("IValidatorRegistry"), Flare).expect("Interface IValidatorRegistry not found");
    let _ = interface_to_abi(String::from("IFtsoRegistry"), Flare).expect("Interface IFtsoRegistry not found");
    let _ = interface_to_abi(String::from("IFtsoGenesis"), Flare).expect("Interface IFtsoGenesis not found");
    let _ = interface_to_abi(String::from("IFlareDaemonize"), Flare).expect("Interface IFlareDaemonize not found");
    let _ = interface_to_abi(String::from("IFtsoRegistryGenesis"), Flare).expect("Interface IFtsoRegistryGenesis not found");
    let _ = interface_to_abi(String::from("IFtsoManagerGenesis"), Flare).expect("Interface IFtsoManagerGenesis not found");
    let _ = interface_to_abi(String::from("IIEntityManager"), Flare).expect("Interface IIEntityManager not found");
    let _ = interface_to_abi(String::from("IICleanupBlockNumberManager"), Flare).expect("Interface IICleanupBlockNumberManager not found");
    let _ = interface_to_abi(String::from("IIRewardManager"), Flare).expect("Interface IIRewardManager not found");
    let _ = interface_to_abi(String::from("IIVoterRegistry"), Flare).expect("Interface IIVoterRegistry not found");
    let _ = interface_to_abi(String::from("IIPublicKeyVerifier"), Flare).expect("Interface IIPublicKeyVerifier not found");
    let _ = interface_to_abi(String::from("IISubmission"), Flare).expect("Interface IISubmission not found");
    let _ = interface_to_abi(String::from("IIVoterRegistrationTrigger"), Flare).expect("Interface IIVoterRegistrationTrigger not found");
    let _ = interface_to_abi(String::from("IIClaimSetupManager"), Flare).expect("Interface IIClaimSetupManager not found");
    let _ = interface_to_abi(String::from("IIRelay"), Flare).expect("Interface IIRelay not found");
    let _ = interface_to_abi(String::from("IIRewardEpochSwitchoverTrigger"), Flare).expect("Interface IIRewardEpochSwitchoverTrigger not found");
    let _ = interface_to_abi(String::from("IIFlareSystemsCalculator"), Flare).expect("Interface IIFlareSystemsCalculator not found");
    let _ = interface_to_abi(String::from("IINodePossessionVerifier"), Flare).expect("Interface IINodePossessionVerifier not found");
    let _ = interface_to_abi(String::from("IIFlareSystemsManager"), Flare).expect("Interface IIFlareSystemsManager not found");
    let _ = interface_to_abi(String::from("IICleanable"), Flare).expect("Interface IICleanable not found");
    let _ = interface_to_abi(String::from("IIVPToken"), Flare).expect("Interface IIVPToken not found");
    let _ = interface_to_abi(String::from("IIVPContract"), Flare).expect("Interface IIVPContract not found");
    let _ = interface_to_abi(String::from("IIGovernanceVotePower"), Flare).expect("Interface IIGovernanceVotePower not found");
    let _ = interface_to_abi(String::from("IIRNatAccount"), Flare).expect("Interface IIRNatAccount not found");
    let _ = interface_to_abi(String::from("IIRNat"), Flare).expect("Interface IIRNat not found");
    let _ = interface_to_abi(String::from("IIFtsoFeedPublisher"), Flare).expect("Interface IIFtsoFeedPublisher not found");
    let _ = interface_to_abi(String::from("IIFtso"), Flare).expect("Interface IIFtso not found");
    let _ = interface_to_abi(String::from("IIPollingFoundation"), Flare).expect("Interface IIPollingFoundation not found");
    let _ = interface_to_abi(String::from("IIGovernorProposer"), Flare).expect("Interface IIGovernorProposer not found");
    let _ = interface_to_abi(String::from("Bn256"), Flare).expect("Interface Bn256 not found");
    let _ = interface_to_abi(String::from("IIFastUpdater"), Flare).expect("Interface IIFastUpdater not found");
    let _ = interface_to_abi(String::from("IIFastUpdateIncentiveManager"), Flare).expect("Interface IIFastUpdateIncentiveManager not found");
}

#[test]
fn abis_for_contracts() {
    let _ = flare::products::WNat;
    let _ = flare::products::PriceSubmitter;
    let _ = flare::products::PChainStakeMirror;
    let _ = flare::products::FlareContractRegistry;
    let _ = flare::products::GovernanceSettings;
    let _ = flare::products::GovernanceVotePower;
    let _ = flare::products::FtsoManager;
    let _ = flare::products::PollingFtso;
    let _ = flare::products::PChainStakeMirrorVerifier;
    let _ = flare::products::ClaimSetupManager;
    let _ = flare::products::DistributionToDelegators;
    let _ = flare::products::ValidatorRewardManager;
    let _ = flare::products::FlareAssetRegistry;
    let _ = flare::products::FtsoRewardManager;
    let _ = flare::products::VoterWhitelister;
    let _ = flare::products::AddressBinder;
    let _ = flare::products::PChainStakeMirrorMultiSigVoting;
    let _ = flare::products::RNat;
    let _ = flare::products::ValidatorRegistry;
    let _ = flare::products::FtsoRegistry;
}

#[test]
fn contract_name_to_abi() {
    let _ = name_to_abi(String::from("WNat"), Flare).expect("Contract WNat not found");
    let _ = name_to_abi(String::from("PriceSubmitter"), Flare).expect("Contract PriceSubmitter not found");
    let _ = name_to_abi(String::from("PChainStakeMirror"), Flare).expect("Contract PChainStakeMirror not found");
    let _ = name_to_abi(String::from("FlareContractRegistry"), Flare).expect("Contract FlareContractRegistry not found");
    let _ = name_to_abi(String::from("GovernanceSettings"), Flare).expect("Contract GovernanceSettings not found");
    let _ = name_to_abi(String::from("GovernanceVotePower"), Flare).expect("Contract GovernanceVotePower not found");
    let _ = name_to_abi(String::from("FtsoManager"), Flare).expect("Contract FtsoManager not found");
    let _ = name_to_abi(String::from("PollingFtso"), Flare).expect("Contract PollingFtso not found");
    let _ = name_to_abi(String::from("PChainStakeMirrorVerifier"), Flare).expect("Contract PChainStakeMirrorVerifier not found");
    let _ = name_to_abi(String::from("ClaimSetupManager"), Flare).expect("Contract ClaimSetupManager not found");
    let _ = name_to_abi(String::from("DistributionToDelegators"), Flare).expect("Contract DistributionToDelegators not found");
    let _ = name_to_abi(String::from("ValidatorRewardManager"), Flare).expect("Contract ValidatorRewardManager not found");
    let _ = name_to_abi(String::from("FlareAssetRegistry"), Flare).expect("Contract FlareAssetRegistry not found");
    let _ = name_to_abi(String::from("FtsoRewardManager"), Flare).expect("Contract FtsoRewardManager not found");
    let _ = name_to_abi(String::from("VoterWhitelister"), Flare).expect("Contract VoterWhitelister not found");
    let _ = name_to_abi(String::from("AddressBinder"), Flare).expect("Contract AddressBinder not found");
    let _ = name_to_abi(String::from("PChainStakeMirrorMultiSigVoting"), Flare).expect("Contract PChainStakeMirrorMultiSigVoting not found");
    let _ = name_to_abi(String::from("RNat"), Flare).expect("Contract RNat not found");
    let _ = name_to_abi(String::from("ValidatorRegistry"), Flare).expect("Contract ValidatorRegistry not found");
    let _ = name_to_abi(String::from("FtsoRegistry"), Flare).expect("Contract FtsoRegistry not found");
}

#[test]
fn toplevel_match() {
    assert_eq!(
        interface_to_abi(String::from("IWNat"), Flare),
        name_to_abi(String::from("WNat"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IPriceSubmitter"), Flare),
        name_to_abi(String::from("PriceSubmitter"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IPChainStakeMirror"), Flare),
        name_to_abi(String::from("PChainStakeMirror"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IFlareContractRegistry"), Flare),
        name_to_abi(String::from("FlareContractRegistry"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IGovernanceSettings"), Flare),
        name_to_abi(String::from("GovernanceSettings"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IGovernanceVotePower"), Flare),
        name_to_abi(String::from("GovernanceVotePower"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoManager"), Flare),
        name_to_abi(String::from("FtsoManager"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IPollingFtso"), Flare),
        name_to_abi(String::from("PollingFtso"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IPChainStakeMirrorVerifier"), Flare),
        name_to_abi(String::from("PChainStakeMirrorVerifier"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IClaimSetupManager"), Flare),
        name_to_abi(String::from("ClaimSetupManager"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IDistributionToDelegators"), Flare),
        name_to_abi(String::from("DistributionToDelegators"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IGenericRewardManager"), Flare),
        name_to_abi(String::from("ValidatorRewardManager"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IFlareAssetRegistry"), Flare),
        name_to_abi(String::from("FlareAssetRegistry"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoRewardManager"), Flare),
        name_to_abi(String::from("FtsoRewardManager"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IVoterWhitelister"), Flare),
        name_to_abi(String::from("VoterWhitelister"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IAddressBinder"), Flare),
        name_to_abi(String::from("AddressBinder"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IPChainStakeMirrorMultiSigVoting"), Flare),
        name_to_abi(String::from("PChainStakeMirrorMultiSigVoting"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IRNat"), Flare),
        name_to_abi(String::from("RNat"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IValidatorRegistry"), Flare),
        name_to_abi(String::from("ValidatorRegistry"), Flare)
    );
    assert_eq!(
        interface_to_abi(String::from("IFtsoRegistry"), Flare),
        name_to_abi(String::from("FtsoRegistry"), Flare)
    );
}

// This file was generated automatically. Do not edit manually.
