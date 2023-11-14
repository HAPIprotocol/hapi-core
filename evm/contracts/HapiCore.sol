// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title HAPI Core EVM
 * @author HAPI Protocol development team
 *
 * Core contract for the HAPI protocol
 */
contract HapiCore is OwnableUpgradeable {
    /// A modifier that allows only the owner or authority to perform a restricted action
    modifier onlyOwnerOrAuthority() {
        require(
            owner() == _msgSender() || _authority == _msgSender(),
            "Caller is not the owner or authority"
        );
        _;
    }

    modifier onlyAuthority() {
        require(_authority == _msgSender(), "Caller is not the authority");
        _;
    }

    modifier onlyPublisherOrAuthorityReporter() {
        ReporterRole role = getMyRole();
        require(
            role == ReporterRole.Publisher || role == ReporterRole.Authority,
            "Caller is not a reporter with the required role"
        );
        _;
    }

    /// Initializes the contract
    function initialize() public initializer {
        __Ownable_init();

        _authority = _msgSender();
    }

    /// Address of the authority that can perform restricted actions (not to be confused with the owner, which should only be used for deployment and upgrades)
    address private _authority;

    /// @param authority New authority address
    event AuthorityChanged(address authority);

    /**
     * Sets the authority address
     * @param newAuthority New authority address
     */
    function setAuthority(address newAuthority) public onlyOwnerOrAuthority {
        _authority = newAuthority;

        emit AuthorityChanged(newAuthority);
    }

    /**
     * Returns the authority address
     * @return Authority address
     */
    function authority() public view virtual returns (address) {
        return _authority;
    }

    /// Stake configuration
    struct StakeConfiguration {
        /// Stake token contract address
        address token;
        /// Duration of reporter suspension before the stake can be withdrawn
        /// @dev The value is in seconds that must pass after the reporter have requested account deactivation
        uint unlock_duration;
        /// Stake amount for Validator reporter
        uint256 validator_stake;
        /// Stake amount for Tracer reporter
        uint256 tracer_stake;
        /// Stake amount for Publisher reporter
        uint256 publisher_stake;
        /// Stake amount for Authority reporter
        uint256 authority_stake;
    }
    StakeConfiguration private _stake_configuration;

    /**
     * @param token Stake token contract address
     * @param unlock_duration Duration of reporter suspension before the stake can be withdrawn
     * @param validator_stake Stake amount for Validator reporter
     * @param tracer_stake Stake amount for Tracer reporter
     * @param publisher_stake Stake amount for Publisher reporter
     * @param authority_stake Stake amount for Authority reporter
     */
    event StakeConfigurationChanged(
        address token,
        uint unlock_duration,
        uint256 validator_stake,
        uint256 tracer_stake,
        uint256 publisher_stake,
        uint256 authority_stake
    );

    /**
     * Update stake configuration
     * @param token Stake token contract address
     * @param unlock_duration Duration of reporter suspension before the stake can be withdrawn
     * @param validator_stake Stake amount for Validator reporter
     * @param tracer_stake Stake amount for Tracer reporter
     * @param publisher_stake Stake amount for Publisher reporter
     * @param authority_stake Stake amount for Authority reporter
     */
    function updateStakeConfiguration(
        address token,
        uint unlock_duration,
        uint256 validator_stake,
        uint256 tracer_stake,
        uint256 publisher_stake,
        uint256 authority_stake
    ) public onlyAuthority {
        _stake_configuration.token = token;
        _stake_configuration.unlock_duration = unlock_duration;
        _stake_configuration.validator_stake = validator_stake;
        _stake_configuration.tracer_stake = tracer_stake;
        _stake_configuration.publisher_stake = publisher_stake;
        _stake_configuration.authority_stake = authority_stake;

        emit StakeConfigurationChanged(
            token,
            unlock_duration,
            validator_stake,
            tracer_stake,
            publisher_stake,
            authority_stake
        );
    }

    /**
     * Returns current stake configuration
     * @return Stake configuration
     * @dev Panics if configuration not set
     */
    function stakeConfiguration()
        public
        view
        virtual
        returns (StakeConfiguration memory)
    {
        require(
            _stake_configuration.token != address(0),
            "Stake configuration is not set"
        );

        return _stake_configuration;
    }

    /// Reward configuration
    struct RewardConfiguration {
        address token;
        /// Address reward amount for Validator reporter
        uint256 address_confirmation_reward;
        /// Address reward amount for Tracer reporter
        uint256 address_tracer_reward;
        /// Asset reward amount for Validator reporter
        uint256 asset_confirmation_reward;
        /// Asset reward amount for Tracer reporter
        uint256 asset_tracer_reward;
    }
    RewardConfiguration private _reward_configuration;

    /**
     * @param token Reward token contract address
     * @param address_confirmation_reward Address reward amount for Validator reporter
     * @param address_tracer_reward Address reward amount for Tracer reporter
     * @param asset_confirmation_reward Asset reward amount for Validator reporter
     * @param asset_tracer_reward Asset reward amount for Tracer reporter
     */
    event RewardConfigurationChanged(
        address token,
        uint256 address_confirmation_reward,
        uint256 address_tracer_reward,
        uint256 asset_confirmation_reward,
        uint256 asset_tracer_reward
    );

    /**
     * Update reward configuration
     * @param token Reward token contract address
     * @param address_confirmation_reward Address reward amount for Validator reporter
     * @param address_tracer_reward Address reward amount for Tracer reporter
     * @param asset_confirmation_reward Asset reward amount for Validator reporter
     * @param asset_tracer_reward Asset reward amount for Tracer reporter
     */
    function updateRewardConfiguration(
        address token,
        uint256 address_confirmation_reward,
        uint256 address_tracer_reward,
        uint256 asset_confirmation_reward,
        uint256 asset_tracer_reward
    ) public onlyAuthority {
        _reward_configuration.token = token;
        _reward_configuration
            .address_confirmation_reward = address_confirmation_reward;
        _reward_configuration.address_tracer_reward = address_tracer_reward;
        _reward_configuration
            .asset_confirmation_reward = asset_confirmation_reward;
        _reward_configuration.asset_tracer_reward = asset_tracer_reward;

        emit RewardConfigurationChanged(
            token,
            address_confirmation_reward,
            address_tracer_reward,
            asset_confirmation_reward,
            asset_tracer_reward
        );
    }

    /**
     * Returns current reward configuration
     * @return Reward configuration
     * @dev Panics if configuration not set
     */
    function rewardConfiguration()
        public
        view
        virtual
        returns (RewardConfiguration memory)
    {
        require(
            _reward_configuration.token != address(0),
            "Reward configuration is not set"
        );

        return _reward_configuration;
    }

    /// Reporter role
    enum ReporterRole {
        /// Validator reporter
        /// @dev This reporter can only confirm addresses/assets submitted by other reporters
        Validator,
        /// Tracer reporter
        /// @dev This reporter can only add address/asset data to existing cases
        Tracer,
        /// Publisher reporter
        /// @dev This is the most common type of reporter
        Publisher,
        /// Authority reporter
        /// @dev This reporter is needed to implement governance decisions on submitted data correction
        Authority
    }

    /// Reporter status
    enum ReporterStatus {
        /// Inactive reporter
        /// @dev Inactive reporter can't submit data and must put a stake to activate
        Inactive,
        /// Active reporter
        Active,
        /// Reporter is in the process of unstaking
        /// @dev Reporter can't submit data and must wait for the unstaking process to withdraw the stake
        Unstaking
    }

    struct Reporter {
        /// Reporter global UUID
        uint128 id;
        /// Reporter address
        address account;
        /// Reporter display name
        string name;
        /// Reporter public page link
        string url;
        /// Reporter role
        ReporterRole role;
        /// Reporter status
        ReporterStatus status;
        /// Reporter stake
        uint256 stake;
        /// Reporter stake unlock timestamp
        uint unlock_timestamp;
    }

    /// A map from reporter UUID to reporter account
    mapping(uint128 => Reporter) private _reporters;
    mapping(address => uint128) private _reporter_ids_by_account;
    uint128[] private _reporter_ids;

    /**
     * @param id Reporter UUID
     * @param reporter Reporter address
     * @param role Reporter role
     */
    event ReporterCreated(
        uint128 indexed id,
        address reporter,
        ReporterRole role
    );

    /**
     * Creates a new reporter
     *
     * @param id Reporter UUID
     * @param account Reporter address
     * @param role Reporter role
     * @param name Reporter display name
     * @param url Reporter public page link
     *
     * @dev Only the authority can create reporters
     * @dev Panics if reporter with the same ID already exists
     */
    function createReporter(
        uint128 id,
        address account,
        ReporterRole role,
        string memory name,
        string memory url
    ) public onlyAuthority {
        require(
            _reporters[id].id == 0,
            "Reporter with the same ID already exists"
        );

        _reporters[id] = Reporter({
            id: id,
            account: account,
            name: name,
            url: url,
            role: role,
            status: ReporterStatus.Inactive,
            stake: 0,
            unlock_timestamp: 0
        });

        _reporter_ids_by_account[account] = id;
        _reporter_ids.push(id);

        emit ReporterCreated(id, account, role);
    }

    /**
     * @param id Reporter UUID
     * @param account Reporter address
     * @param role Reporter role
     */
    event ReporterUpdated(
        uint128 indexed id,
        address account,
        ReporterRole role
    );

    /**
     * Updates an existing reporter
     *
     * @param id Reporter UUID
     * @param account Reporter address
     * @param role Reporter role
     * @param name Reporter display name
     * @param url Reporter public page link
     *
     * @dev Only the authority can update reporters
     * @dev Panics if the reporter does not exist
     */
    function updateReporter(
        uint128 id,
        address account,
        ReporterRole role,
        string memory name,
        string memory url
    ) public onlyAuthority {
        require(
            _reporters[id].id > 0,
            "Reporter with the same ID does not exist"
        );

        Reporter storage reporter = _reporters[id];

        delete _reporter_ids_by_account[reporter.account];

        reporter.role = role;
        reporter.account = account;
        reporter.name = name;
        reporter.url = url;

        _reporter_ids_by_account[account] = id;

        emit ReporterUpdated(id, account, role);
    }

    /**
     * Retrieves caller's reporter ID
     */
    function getMyReporterId() public view returns (uint128) {
        return _reporter_ids_by_account[_msgSender()];
    }

    /**
     * Retrieves caller's reporter role
     *
     * @dev Panics if the caller is not a reporter
     */
    function getMyRole() public view returns (ReporterRole) {
        uint128 id = getMyReporterId();

        require(id > 0, "Caller is not a reporter");
        require(
            _reporters[id].status == ReporterStatus.Active,
            "Reporter is not active"
        );

        return _reporters[id].role;
    }

    /**
     * Retrieves reporter data
     *
     * @param id Reporter UUID
     *
     * @dev Panics if the reporter does not exist
     */
    function getReporter(uint128 id) public view returns (Reporter memory) {
        require(_reporters[id].id > 0, "Reporter does not exist");

        return _reporters[id];
    }

    /**
     * Retrieves paged reporter list
     *
     * @param skip Number of reporters to skip
     * @param take Number of reporters to retrieve
     */
    function getReporters(
        uint skip,
        uint take
    ) public view returns (Reporter[] memory) {
        uint length = _reporter_ids.length;

        if (skip >= length) {
            return new Reporter[](0);
        }

        uint size = take;

        if (size > length - skip) {
            size = length - skip;
        }

        Reporter[] memory reporters = new Reporter[](size);

        for (uint i = 0; i < size; i++) {
            reporters[i] = _reporters[_reporter_ids[skip + i]];
        }

        return reporters;
    }

    /**
     * Retrieves reporter count
     */
    function getReporterCount() public view virtual returns (uint) {
        return _reporter_ids.length;
    }

    /**
     * @param id Reporter UUID
     */
    event ReporterActivated(uint128 indexed id);

    /**
     * Activates a reporter
     *
     * @dev Panics if the caller is not a reporter
     * @dev Panics if the reporter is not inactive
     * @dev Panics if the reporter role stake is not configured
     * @dev Panics if the caller does not have enough tokens or haven't set up allowance to stake
     */
    function activateReporter() external {
        uint128 id = getMyReporterId();

        require(id > 0, "Caller is not a reporter");

        Reporter storage reporter = _reporters[id];

        require(
            reporter.status == ReporterStatus.Inactive,
            "Reporter is not inactive"
        );

        uint256 amount = 0;

        if (reporter.role == ReporterRole.Validator) {
            amount = _stake_configuration.validator_stake;
        } else if (reporter.role == ReporterRole.Publisher) {
            amount = _stake_configuration.publisher_stake;
        } else if (reporter.role == ReporterRole.Tracer) {
            amount = _stake_configuration.tracer_stake;
        } else if (reporter.role == ReporterRole.Authority) {
            amount = _stake_configuration.authority_stake;
        }

        require(amount > 0, "Reporter role stake is not configured");

        require(
            IERC20(_stake_configuration.token).transferFrom(
                msg.sender,
                address(this),
                amount
            ),
            "Insufficient tokens or allowance to stake"
        );

        reporter.status = ReporterStatus.Active;
        reporter.stake = amount;

        emit ReporterActivated(id);
    }

    event ReporterDeactivated(uint128 indexed id);

    /**
     * Deactivate reporter for unstaking after the unlock period
     *
     * @dev Panics if the caller is not a reporter
     * @dev Panics if the reporter is not active
     */
    function deactivateReporter() external {
        uint128 id = getMyReporterId();

        require(id > 0, "Caller is not a reporter");

        Reporter storage reporter = _reporters[id];

        require(
            reporter.status == ReporterStatus.Active,
            "Reporter is not active"
        );

        reporter.status = ReporterStatus.Unstaking;
        reporter.unlock_timestamp =
            block.timestamp +
            _stake_configuration.unlock_duration;

        emit ReporterDeactivated(id);
    }

    /**
     * @param id Reporter UUID
     */
    event ReporterStakeWithdrawn(uint128 indexed id);

    /**
     * Unstake tokens by the reporter after the unlock period
     *
     * @dev Panics if the caller is not a reporter
     * @dev Panics if the reporter is not unstaking
     * @dev Panics if the reporter is not unlocked yet
     */
    function unstake() external {
        uint128 id = getMyReporterId();

        require(id > 0, "Caller is not a reporter");

        Reporter storage reporter = _reporters[id];

        require(
            reporter.status == ReporterStatus.Unstaking,
            "Reporter is not unstaking"
        );
        require(
            reporter.unlock_timestamp <= block.timestamp,
            "Reporter is not unlocked yet"
        );

        // NOTE: Situation where there's not enough tokens to withdraw should be impossible,
        // as the pool is only formed from the tokens staked by the reporters
        require(
            IERC20(_stake_configuration.token).transfer(
                msg.sender,
                reporter.stake
            )
        );

        reporter.status = ReporterStatus.Inactive;
        reporter.stake = 0;
        reporter.unlock_timestamp = 0;
    }

    enum CaseStatus {
        /// Case is closed for new data
        Closed,
        /// Case is open for new data
        Open
    }

    struct Case {
        /// Case UUID
        uint128 id;
        /// Case name
        string name;
        /// The UUID of the reporter that created the case
        uint128 reporter_id;
        /// Case status
        CaseStatus status;
        /// Case public page link
        string url;
    }

    /// A map from case UUID to case record
    mapping(uint128 => Case) private _cases;

    /// A list of all case ids
    uint128[] private _case_ids;

    /**
     * @param id Case UUID
     */
    event CaseCreated(uint128 indexed id);

    /**
     * Creates a new case
     *
     * @param id Case UUID
     * @param name Case name
     * @param url Case public page link
     *
     * @dev Panics if the caller is not a reporter
     * @dev Panics if the case with the same ID already exists
     */
    function createCase(
        uint128 id,
        string memory name,
        string memory url
    ) public onlyPublisherOrAuthorityReporter {
        uint128 reporter_id = getMyReporterId();

        require(reporter_id > 0, "Caller is not a reporter");
        require(_cases[id].id == 0, "Case with the same ID already exists");

        _cases[id] = Case({
            id: id,
            name: name,
            reporter_id: reporter_id,
            status: CaseStatus.Open,
            url: url
        });

        _case_ids.push(id);

        emit CaseCreated(id);
    }

    /**
     * @param id Case UUID
     */
    event CaseUpdated(uint128 indexed id);

    /**
     * Updates an existing case
     *
     * @param id Case UUID
     * @param name Case name
     * @param url Case public page link
     * @param status Case status
     *
     * @dev Panics if the caller is not a reporter
     * @dev Panics if the case does not exist
     * @dev Panics if the caller is not the case reporter or authority
     */
    function updateCase(
        uint128 id,
        string memory name,
        string memory url,
        CaseStatus status
    ) public onlyPublisherOrAuthorityReporter {
        Case storage case_record = _cases[id];

        require(case_record.id > 0, "Case does not exist");

        require(
            case_record.reporter_id == getMyReporterId() ||
                getMyRole() == ReporterRole.Authority,
            "Must be the case reporter or authority"
        );

        case_record.name = name;
        case_record.url = url;
        case_record.status = status;

        emit CaseUpdated(id);
    }

    /**
     * Retrieves case data
     *
     * @param id Case UUID
     *
     * @dev Panics if the case does not exist
     */
    function getCase(uint128 id) public view virtual returns (Case memory) {
        require(_cases[id].id > 0, "Case does not exist");

        return _cases[id];
    }

    /**
     * Retrieves paged case list
     *
     * @param skip Number of cases to skip
     * @param take Number of cases to retrieve
     */
    function getCases(
        uint skip,
        uint take
    ) public view virtual returns (Case[] memory) {
        uint length = _case_ids.length;

        if (skip >= length) {
            return new Case[](0);
        }

        uint size = take;

        if (size > length - skip) {
            size = length - skip;
        }

        Case[] memory cases = new Case[](size);

        for (uint i = 0; i < size; i++) {
            cases[i] = _cases[_case_ids[skip + i]];
        }

        return cases;
    }

    /**
     * Retrieves case count
     */
    function getCaseCount() public view virtual returns (uint) {
        return _case_ids.length;
    }

    enum Category {
        None,
        WalletService,
        MerchantService,
        MiningPool,
        Exchange,
        DeFi,
        OTCBroker,
        ATM,
        Gambling,
        IllicitOrganization,
        Mixer,
        DarknetService,
        Scam,
        Ransomware,
        Theft,
        Counterfeit,
        TerroristFinancing,
        Sanctions,
        ChildAbuse,
        Hacker,
        HighRiskJurisdiction
    }

    struct Address {
        /// The address
        address addr;
        /// The UUID of address' case
        uint128 case_id;
        /// The UUID of the reporter that submitted the address
        uint128 reporter_id;
        /// The number of confirmations for the address
        uint confirmations;
        /// Risk score for the address (0..10)
        uint8 risk;
        /// Category of activity associated with the address
        Category category;
    }

    /// A map from address to address record
    mapping(address => Address) private _addresses;

    /// A list of all addresses
    address[] private _address_addrs;

    // Mapping to keep track of address confirmations
    mapping(address => mapping(uint128 => bool)) private _address_confirmations;

    /**
     * @param addr Address
     * @param risk Risk score for the address (0..10)
     * @param category Category of activity associated with the address
     */
    event AddressCreated(address indexed addr, uint8 risk, Category category);

    /**
     * Creates a new address
     *
     * @param addr Address
     * @param case_id Case UUID
     * @param risk Risk score for the address (0..10)
     * @param category Category of activity associated with the address
     *
     * @dev Panics if the case does not exist
     * @dev Panics if the address already exists
     * @dev Panics if the risk is not between 0 and 10
     * @dev Panics if the caller is not a reporter with the required role
     */
    function createAddress(
        address addr,
        uint128 case_id,
        uint8 risk,
        Category category
    ) public {
        require(_cases[case_id].id > 0, "Case does not exist");
        require(_cases[case_id].status == CaseStatus.Open, "Case is closed");
        require(_addresses[addr].addr == address(0), "Address already exists");
        require(risk >= 0 && risk <= 10, "Risk must be between 0 and 10");

        uint128 reporter_id = getMyReporterId();
        ReporterRole role = getMyRole();

        require(
            role == ReporterRole.Publisher ||
                role == ReporterRole.Authority ||
                role == ReporterRole.Tracer,
            "Caller is not a reporter with the required role"
        );

        _addresses[addr] = Address({
            addr: addr,
            case_id: case_id,
            reporter_id: reporter_id,
            confirmations: 0,
            risk: risk,
            category: category
        });

        _address_addrs.push(addr);

        emit AddressCreated(addr, risk, category);
    }

    /**
     * @param addr Address
     * @param risk Risk score for the address (0..10)
     * @param category Category of activity associated with the address
     */
    event AddressUpdated(address indexed addr, uint8 risk, Category category);

    /**
     * Updates an existing address
     *
     * @param addr Address
     * @param risk Risk score for the address (0..10)
     * @param category Category of activity associated with the address
     * @param case_id Case UUID
     *
     * @dev Panics if the address does not exist
     * @dev Panics if the risk is not between 0 and 10
     * @dev Panics if the case does not exist
     * @dev Panics if the caller is not the address reporter or authority
     * @dev Panics if the caller is a tracer and tries to change the case
     */
    function updateAddress(
        address addr,
        uint8 risk,
        Category category,
        uint128 case_id
    ) public {
        require(_addresses[addr].addr != address(0), "Address does not exist");
        require(risk >= 0 && risk <= 10, "Risk must be between 0 and 10");
        require(_cases[case_id].id > 0, "Case does not exist");

        uint128 reporter_id = getMyReporterId();
        ReporterRole role = getMyRole();

        require(
            _addresses[addr].reporter_id == reporter_id ||
                role == ReporterRole.Authority,
            "Caller is not the address reporter or authority"
        );

        if (_addresses[addr].case_id != case_id) {
            require(role != ReporterRole.Tracer, "Tracer can't change case");
            _addresses[addr].case_id = case_id;
        }

        _addresses[addr].risk = risk;
        _addresses[addr].category = category;

        emit AddressUpdated(addr, risk, category);
    }

    /**
     * @param addr Address
     */
    event AddressConfirmed(address indexed addr);

    /**
     * Updates an existing address
     *
     * @param addr Address
     *
     * @dev Panics if the address does not exist
     * @dev Panics if the caller is not a publisher or a validator
     * @dev Panics if the caller already confirmed the address
     */
    function confirmAddress(address addr) public {
        require(_addresses[addr].addr != address(0), "Address does not exist");

        uint128 reporter_id = getMyReporterId();
        ReporterRole role = getMyRole();

        require(
            role == ReporterRole.Publisher || role == ReporterRole.Validator,
            "Reporter is not publisher or validator"
        );
        require(
            !_address_confirmations[addr][reporter_id],
            "The reporter has already confirmed the address"
        );

        _address_confirmations[addr][reporter_id] = true;
        _addresses[addr].confirmations++;

        emit AddressConfirmed(addr);
    }

    /**
     * Retrieves address data
     *
     * @param addr Address
     *
     * @dev Returns an empty record for addresses that don't exist
     */
    function getAddress(
        address addr
    ) public view virtual returns (Address memory) {
        return _addresses[addr];
    }

    /**
     * Retrieves address count
     */
    function getAddressCount() public view virtual returns (uint) {
        return _address_addrs.length;
    }

    /**
     * Retrieves paged address list
     *
     * @param skip Number of addresses to skip
     * @param take Number of addresses to retrieve
     */
    function getAddresses(
        uint skip,
        uint take
    ) public view virtual returns (Address[] memory) {
        uint length = _address_addrs.length;

        if (skip >= length) {
            return new Address[](0);
        }

        uint size = take;

        if (size > length - skip) {
            size = length - skip;
        }

        Address[] memory addresses = new Address[](size);

        for (uint i = 0; i < size; i++) {
            addresses[i] = _addresses[_address_addrs[skip + i]];
        }

        return addresses;
    }

    struct Asset {
        /// Asset contract address
        address addr;
        /// Asset ID (ERC-721 compatible)
        uint256 asset_id;
        /// The UUID of address' case
        uint128 case_id;
        /// The UUID of the reporter that submitted the address
        uint128 reporter_id;
        /// The number of confirmations for the address
        uint confirmations;
        /// Risk score for the address (0..10)
        uint8 risk;
        /// Category of activity associated with the address
        Category category;
    }

    struct AssetKey {
        address addr;
        uint256 asset_id;
    }

    /// A map from address and asset ID to asset record
    mapping(address => mapping(uint256 => Asset)) private _assets;

    /// A list of all assets
    AssetKey[] private _asset_addrs;

    // Mapping to keep track of asset confirmations
    mapping(address => mapping(uint256 => mapping(uint256 => bool)))
        private _asset_confirmations;

    /**
     * @param addr Asset contract address
     * @param asset_id Asset ID (ERC-721 compatible)
     * @param risk Risk score for the address (0..10)
     * @param category Category of activity associated with the address
     */
    event AssetCreated(
        address indexed addr,
        uint256 asset_id,
        uint8 risk,
        Category category
    );

    /**
     * Creates a new asset
     *
     * @param addr Asset contract address
     * @param asset_id Asset ID (ERC-721 compatible)
     * @param case_id Case UUID
     * @param risk Risk score for the address (0..10)
     * @param category Category of activity associated with the address
     *
     * @dev Panics if the case does not exist
     * @dev Panics if the address already exists
     * @dev Panics if the risk is not between 0 and 10
     * @dev Panics if the caller is not a reporter with the required role
     */
    function createAsset(
        address addr,
        uint256 asset_id,
        uint128 case_id,
        uint8 risk,
        Category category
    ) public {
        require(_cases[case_id].id > 0, "Case does not exist");
        require(_cases[case_id].status == CaseStatus.Open, "Case is closed");
        require(
            _assets[addr][asset_id].addr == address(0),
            "Asset already exists"
        );
        require(risk >= 0 && risk <= 10, "Risk must be between 0 and 10");

        uint128 reporter_id = getMyReporterId();
        ReporterRole role = getMyRole();

        require(
            role == ReporterRole.Publisher ||
                role == ReporterRole.Authority ||
                role == ReporterRole.Tracer,
            "Caller is not a reporter with the required role"
        );

        _assets[addr][asset_id] = Asset({
            addr: addr,
            asset_id: asset_id,
            case_id: case_id,
            reporter_id: reporter_id,
            confirmations: 0,
            risk: risk,
            category: category
        });

        _asset_addrs.push(AssetKey({addr: addr, asset_id: asset_id}));

        emit AssetCreated(addr, asset_id, risk, category);
    }

    /**
     * @param addr Asset contract address
     * @param asset_id Asset ID (ERC-721 compatible)
     * @param risk Risk score for the address (0..10)
     * @param category Category of activity associated with the address
     */
    event AssetUpdated(
        address indexed addr,
        uint256 asset_id,
        uint8 risk,
        Category category
    );

    /**
     * Updates an existing address
     *
     * @param addr Asset contract address
     * @param asset_id Asset ID (ERC-721 compatible)
     * @param risk Risk score for the address (0..10)
     * @param category Category of activity associated with the address
     * @param case_id Case UUID
     *
     * @dev Panics if the address does not exist
     * @dev Panics if the risk is not between 0 and 10
     * @dev Panics if the case does not exist
     * @dev Panics if the caller is not the address reporter or authority
     * @dev Panics if the caller is a tracer and tries to change the case
     */
    function updateAsset(
        address addr,
        uint256 asset_id,
        uint8 risk,
        Category category,
        uint128 case_id
    ) public {
        require(
            _assets[addr][asset_id].addr != address(0),
            "Address does not exist"
        );
        require(risk >= 0 && risk <= 10, "Risk must be between 0 and 10");
        require(_cases[case_id].id > 0, "Case does not exist");

        uint128 reporter_id = getMyReporterId();
        ReporterRole role = getMyRole();

        require(
            _assets[addr][asset_id].reporter_id == reporter_id ||
                role == ReporterRole.Authority,
            "Caller is not the address reporter or authority"
        );

        if (_assets[addr][asset_id].case_id != case_id) {
            require(role != ReporterRole.Tracer, "Tracer can't change case");
            _assets[addr][asset_id].case_id = case_id;
        }

        _assets[addr][asset_id].risk = risk;
        _assets[addr][asset_id].category = category;

        emit AssetUpdated(addr, asset_id, risk, category);
    }

    /**
     * @param addr Asset contract address
     * @param asset_id Asset ID (ERC-721 compatible)
     */
    event AssetConfirmed(address indexed addr, uint256 asset_id);

    /**
     * Updates an existing address
     *
     * @param addr Asset contract address
     * @param asset_id Asset ID (ERC-721 compatible)
     *
     * @dev Panics if the asset does not exist
     * @dev Panics if the caller is not a publisher or a validator
     * @dev Panics if the caller already confirmed the asset
     */
    function confirmAsset(address addr, uint256 asset_id) public {
        require(
            _assets[addr][asset_id].addr != address(0),
            "Address does not exist"
        );

        uint128 reporter_id = getMyReporterId();
        ReporterRole role = getMyRole();

        require(
            role == ReporterRole.Publisher || role == ReporterRole.Validator,
            "Reporter is not publisher or validator"
        );
        require(
            !_asset_confirmations[addr][asset_id][reporter_id],
            "The reporter has already confirmed the asset"
        );

        _asset_confirmations[addr][asset_id][reporter_id] = true;
        _assets[addr][asset_id].confirmations++;

        emit AssetConfirmed(addr, asset_id);
    }

    /**
     * Retrieves asset data
     *
     * @param addr Asset contract address
     * @param asset_id Asset ID (ERC-721 compatible)
     *
     * @dev Returns an empty record for addresses that don't exist
     */
    function getAsset(
        address addr,
        uint256 asset_id
    ) public view virtual returns (Asset memory) {
        return _assets[addr][asset_id];
    }

    /**
     * Retrieves asset count
     */
    function getAssetCount() public view virtual returns (uint) {
        return _asset_addrs.length;
    }

    /**
     * Retrieves paged asset list
     *
     * @param skip Number of addresses to skip
     * @param take Number of addresses to retrieve
     */
    function getAssets(
        uint skip,
        uint take
    ) public view virtual returns (Asset[] memory) {
        uint length = _asset_addrs.length;

        if (skip >= length) {
            return new Asset[](0);
        }

        uint size = take;

        if (size > length - skip) {
            size = length - skip;
        }

        Asset[] memory assets = new Asset[](size);

        for (uint i = 0; i < size; i++) {
            AssetKey memory key = _asset_addrs[skip + i];
            assets[i] = _assets[key.addr][key.asset_id];
        }

        return assets;
    }
}
