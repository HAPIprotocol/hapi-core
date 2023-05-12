// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title HAPI Core
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
     */
    function stakeConfiguration()
        public
        view
        virtual
        returns (StakeConfiguration memory)
    {
        return _stake_configuration;
    }

    /// Reward configuration
    struct RewardConfiguration {
        address token;
        /// Reward amount for Validator reporter
        uint256 address_confirmation_reward;
        /// Reward amount for Tracer reporter
        uint256 tracer_reward;
    }
    RewardConfiguration private _reward_configuration;

    /**
     * @param token Reward token contract address
     * @param address_confirmation_reward Reward amount for Validator reporter
     * @param tracer_reward Reward amount for Tracer reporter
     */
    event RewardConfigurationChanged(
        address token,
        uint256 address_confirmation_reward,
        uint256 tracer_reward
    );

    /**
     * Update reward configuration
     * @param token Reward token contract address
     * @param address_confirmation_reward Reward amount for Validator reporter
     * @param tracer_reward Reward amount for Tracer reporter
     */
    function updateRewardConfiguration(
        address token,
        uint256 address_confirmation_reward,
        uint256 tracer_reward
    ) public onlyAuthority {
        _reward_configuration.token = token;
        _reward_configuration
            .address_confirmation_reward = address_confirmation_reward;
        _reward_configuration.tracer_reward = tracer_reward;

        emit RewardConfigurationChanged(
            token,
            address_confirmation_reward,
            tracer_reward
        );
    }

    /**
     * Returns current reward configuration
     * @return Reward configuration
     */
    function rewardConfiguration()
        public
        view
        virtual
        returns (RewardConfiguration memory)
    {
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
    uint128[] private _reporter_ids;

    /**
     * @param id Reporter UUID
     * @param reporter Reporter address
     * @param role Reporter role
     */
    event ReporterCreated(uint128 indexed id, address reporter, ReporterRole role);

    /**
     * Creates a new reporter
     *
     * @param id Reporter UUID
     * @param account Reporter address
     * @param role Reporter role
     * @param name Reporter display name
     * @param url Reporter public page link
     */
    function createReporter(
        uint128 id,
        address account,
        ReporterRole role,
        string memory name,
        string memory url
    ) public onlyAuthority {
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

        _reporter_ids.push(id);

        emit ReporterCreated(id, account, role);
    }

    /**
     * @param id Reporter UUID
     * @param account Reporter address
     * @param role Reporter role
     */
    event ReporterUpdated(uint128 indexed id, address account, ReporterRole role);

    /**
     * Updates an existing reporter
     *
     * @param id Reporter UUID
     * @param account Reporter address
     * @param role Reporter role
     * @param name Reporter display name
     * @param url Reporter public page link
     */
    function updateReporter(
        uint128 id,
        address account,
        ReporterRole role,
        string memory name,
        string memory url
    ) public onlyAuthority {
        Reporter storage reporter = _reporters[id];

        reporter.role = role;
        reporter.account = account;
        reporter.name = name;
        reporter.url = url;

        emit ReporterUpdated(id, account, role);
    }

    /**
     * Retrieves reporter data
     *
     * @param id Reporter UUID
     */
    function getReporter(
        uint128 id
    ) public view virtual returns (Reporter memory) {
        return _reporters[id];
    }

    /**
     * Retrieves reporter data
     *
     * @param take Number of reporters to retrieve
     * @param skip Number of reporters to skip
     */
    function getReporters(uint take, uint skip) public view virtual returns (Reporter[] memory) {
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
     * @param id Reporter UUID
     */
    function activateReporter(
        uint128 id
    ) external {
        Reporter storage reporter = _reporters[id];

        require(reporter.account == _msgSender(), "Caller is not the target reporter");
        require(reporter.status == ReporterStatus.Inactive, "Reporter is not inactive");

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

        require(amount > 0, "Reporter role is not configured");

        require(IERC20(_stake_configuration.token).transferFrom(msg.sender, address(this), amount));

        reporter.status = ReporterStatus.Active;
        reporter.stake = amount;

        emit ReporterActivated(id);
    }

    event ReporterDeactivated(uint128 indexed id);

    /**
     * Deactivate reporter for unstaking after the unlock period
     * 
     * @param id Reporter UUID
     */
    function deactivateReporter(
        uint128 id
    ) external {
        Reporter storage reporter = _reporters[id];

        require(reporter.account == _msgSender(), "Caller is not the target reporter");
        require(reporter.status == ReporterStatus.Active, "Reporter is not active");

        reporter.status = ReporterStatus.Unstaking;
        reporter.unlock_timestamp = block.timestamp + _stake_configuration.unlock_duration;

        emit ReporterDeactivated(id);
    }
}
