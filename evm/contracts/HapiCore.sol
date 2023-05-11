// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

contract HapiCore is OwnableUpgradeable {
    address private _authority;

    modifier onlyOwnerOrAuthority() {
        require(owner() == _msgSender() || _authority == _msgSender(), "Caller is not the owner or authority");
        _;
    }

    function initialize() public initializer {
        __Ownable_init();

        _authority = _msgSender();
    }

    event AuthorityChanged(address authority);

    function setAuthority(address newAuthority) public onlyOwnerOrAuthority {
        _authority = newAuthority;

        emit AuthorityChanged(newAuthority);
    }

    function authority() public view virtual returns (address) {
        return _authority;
    }
}
