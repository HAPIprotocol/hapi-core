import { AnchorProvider, web3, BN } from "@coral-xyz/anchor";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import * as Token from "@solana/spl-token";

export class TestToken {
  public token: web3.PublicKey;

  constructor(
    private readonly provider: AnchorProvider,
    private readonly decimals: number = 9
  ) {}

  get programId() {
    return Token.TOKEN_PROGRAM_ID;
  }

  get mintAccount() {
    return this.token;
  }

  get payer() {
    if (this.provider.wallet instanceof NodeWallet) {
      return this.provider.wallet["payer"];
    } else {
      throw new Error(`NodeWallet compatible provider expected`);
    }
  }

  async mint(amount?: number): Promise<void> {
    const mint = await Token.createMint(
      this.provider.connection,
      this.payer,
      this.payer.publicKey,
      null,
      this.decimals,
      undefined,
      undefined,
      Token.TOKEN_PROGRAM_ID
    );

    const fromTokenAccount = await Token.getOrCreateAssociatedTokenAccount(
      this.provider.connection,
      this.payer,
      mint,
      this.payer.publicKey
    );

    if (amount !== undefined) {
      await Token.mintTo(
        this.provider.connection,
        this.payer,
        mint,
        fromTokenAccount.address,
        this.payer.publicKey,
        amount
      );
    }

    this.token = mint;
  }

  async createAccount(owner?: web3.PublicKey): Promise<web3.PublicKey> {
    if (owner === undefined) {
      owner = this.payer.publicKey;
    }

    const initialOwner = web3.Keypair.generate();

    const account = await Token.createAssociatedTokenAccount(
      this.provider.connection,
      this.payer,
      this.token,
      initialOwner.publicKey
    );

    await Token.setAuthority(
      this.provider.connection,
      this.payer,
      account,
      initialOwner,
      Token.AuthorityType.AccountOwner,
      owner,
      []
    );

    return account;
  }

  async transfer(
    from: web3.Keypair | web3.Signer | null,
    to: web3.PublicKey,
    amount: number
  ): Promise<void> {
    if (from === null) {
      from = this.payer;
    }

    const fromTokenAccount = await Token.getOrCreateAssociatedTokenAccount(
      this.provider.connection,
      this.payer,
      this.token,
      from.publicKey
    );

    const toTokenAccount = await Token.getOrCreateAssociatedTokenAccount(
      this.provider.connection,
      this.payer,
      this.token,
      to
    );

    await Token.transfer(
      this.provider.connection,
      this.payer,
      fromTokenAccount.address,
      toTokenAccount.address,
      from,
      amount
    );
  }

  async getBalance(
    account: web3.PublicKey,
    allowOwnerOffCurve = false
  ): Promise<BN> {
    const tokenAccount = await Token.getOrCreateAssociatedTokenAccount(
      this.provider.connection,
      this.payer,
      this.token,
      account,
      allowOwnerOffCurve
    );

    const { value } = await this.provider.connection.getTokenAccountBalance(
      tokenAccount.address
    );
    return new BN(value.amount);
  }

  async getTokenAccount(
    account: web3.PublicKey,
    allowOwnerOffCurve = false
  ): Promise<web3.PublicKey> {
    const { address } = await Token.getOrCreateAssociatedTokenAccount(
      this.provider.connection,
      this.payer,
      this.token,
      account,
      allowOwnerOffCurve
    );

    return address;
  }
}
