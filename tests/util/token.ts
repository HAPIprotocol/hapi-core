import { Provider, web3 } from "@project-serum/anchor";
import { NodeWallet } from "@project-serum/anchor/dist/cjs/provider";
import { Token, TOKEN_PROGRAM_ID, u64, AuthorityType } from "@solana/spl-token";

export { Token, u64 } from "@solana/spl-token";

export class TestToken {
  public token: Token;

  constructor(
    private readonly provider: Provider,
    private readonly decimals: number = 9
  ) {}

  get programId() {
    return this.token.programId;
  }

  get mintAccount() {
    return this.token.publicKey;
  }

  get payer() {
    return (this.provider.wallet as NodeWallet).payer;
  }

  async mint(amount: u64): Promise<void> {
    const mint = await Token.createMint(
      this.provider.connection,
      this.payer,
      this.payer.publicKey,
      null,
      this.decimals,
      TOKEN_PROGRAM_ID
    );

    const fromTokenAccount = await mint.getOrCreateAssociatedAccountInfo(
      this.payer.publicKey
    );

    await mint.mintTo(
      fromTokenAccount.address,
      this.payer.publicKey,
      [],
      amount
    );

    this.token = mint;
  }

  async createAccount(owner?: web3.PublicKey): Promise<web3.PublicKey> {
    if (owner === undefined) {
      owner = this.payer.publicKey;
    }

    const account = await this.token.createAccount(this.payer.publicKey);
    await this.token.setAuthority(
      account,
      owner,
      "AccountOwner",
      this.payer,
      []
    );

    return account;
  }

  async transfer(
    from: web3.Keypair | web3.Signer | null,
    to: web3.PublicKey,
    amount: u64
  ): Promise<void> {
    if (from === null) {
      from = this.token.payer;
    }

    const fromTokenAccount = await this.token.getOrCreateAssociatedAccountInfo(
      from.publicKey
    );

    const toTokenAccount = await this.token.getOrCreateAssociatedAccountInfo(
      to
    );

    await this.token.transfer(
      fromTokenAccount.address,
      toTokenAccount.address,
      from,
      [],
      amount
    );
  }

  async getBalance(account: web3.PublicKey): Promise<u64> {
    const tokenAccount = await this.token.getOrCreateAssociatedAccountInfo(
      account
    );
    const { value } = await this.provider.connection.getTokenAccountBalance(
      tokenAccount.address
    );
    return new u64(value.amount);
  }

  async getTokenAccount(account: web3.PublicKey): Promise<web3.PublicKey> {
    const { address } = await this.token.getOrCreateAssociatedAccountInfo(
      account
    );

    return address;
  }
}
