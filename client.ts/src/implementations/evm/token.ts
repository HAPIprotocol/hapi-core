import { Contract } from "@ethersproject/contracts";
import type { Provider } from "@ethersproject/providers";
import type { Signer } from "@ethersproject/abstract-signer";

export function getTokenContract(
  tokenAddress: string,
  provider: Signer | Provider
) {
  const abi = [
    // ERC20 Optional
    "function name() view returns (string)",
    "function symbol() view returns (string)",

    // ERC20 Required
    "function totalSupply() view returns (uint256)",
    "function balanceOf(address) view returns (uint256)",
    "function transfer(address, uint256) returns (boolean)",
    "function allowance(address, address) view returns (uint256)",
    "function approve(address, uint256) returns (boolean)",
    "function transferFrom(address, address, uint256) returns (boolean)",
    "event Transfer(address indexed from, address indexed to, uint256 value)",
    "event Approval(address indexed owner, address indexed spender, uint256 value)",
  ];

  return new Contract(tokenAddress, abi, provider);
}
