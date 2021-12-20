export const ReporterRole = {
  Validator: { validator: {} },
  Tracer: { tracer: {} },
  Full: { full: {} },
  Authority: { authority: {} },
};

export type ReporterRoleType = "Validator" | "Tracer" | "Full" | "Authority";

export const ReporterStatus = {
  Inactive: { inactive: {} },
  Active: { active: {} },
  Unstaking: { unstaking: {} },
};

export type ReporterStatusType = "Inactive" | "Active" | "Unstaking";

export const CaseStatus = {
  Closed: { closed: {} },
  Open: { open: {} },
};

export type CaseStatusType = "Closed" | "Open";

export const Category = {
  None: { none: {} },
  WalletService: { walletService: {} },
  MerchantService: { merchantService: {} },
  MiningPool: { miningPool: {} },
  LowRiskExchange: { lowRiskExchange: {} },
  MediumRiskExchange: { mediumRiskExchange: {} },
  DeFi: { deFi: {} },
  OTCBroker: { oTCBroker: {} },
  ATM: { aTM: {} },
  Gambling: { gambling: {} },
  IllicitOrganization: { illicitOrganization: {} },
  Mixer: { mixer: {} },
  DarknetService: { darknetService: {} },
  Scam: { scam: {} },
  Ransomware: { ransomware: {} },
  Theft: { theft: {} },
  Counterfeit: { counterfeit: {} },
  TerroristFinancing: { terroristFinancing: {} },
  Sanctions: { sanctions: {} },
  ChildAbuse: { childAbuse: {} },
};

export type CategoryType =
  | "None"
  | "WalletService"
  | "MerchantService"
  | "MiningPool"
  | "LowRiskExchange"
  | "MediumRiskExchange"
  | "DeFi"
  | "OTCBroker"
  | "ATM"
  | "Gambling"
  | "IllicitOrganization"
  | "Mixer"
  | "DarknetService"
  | "Scam"
  | "Ransomware"
  | "Theft"
  | "Counterfeit"
  | "TerroristFinancing"
  | "Sanctions"
  | "ChildAbuse";
