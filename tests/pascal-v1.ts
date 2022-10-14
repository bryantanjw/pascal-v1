import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { PascalV1 } from "../target/types/pascal_v1";
import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token";

describe("anchor-movie-review-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PascalV1 as Program<PascalV1>;

  const order = {
    marketId: "Just a test movie",
    description: "Wow, good movie!",
    contracts: 5
  };

  const [order_pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(order.marketId), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  );

  it("Initialized the reward token", async () => {
    const tx = await program.methods
      .initializeTokenMint()
      .accounts({
        mint: mint,
      })
      .rpc();
  })

  it("Movie review is added", async () => {
    const tokenAccount = await getAssociatedTokenAddress(mint, provider.wallet.publicKey)
    const tx = await program.methods
    .placeOrder(order.marketId, order.description, order.contracts)
    .accounts({
      movieReview: order_pda,
      mint: mint,
      tokenAccount: tokenAccount
    })
    .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.orderAccountState.fetch(order_pda);
    expect(order.marketId === account.marketId);
    expect(order.description === account.description);
    expect(order.contracts === account.contracts);
    expect(provider.wallet.publicKey === account.trader);

    const userAta = await getAccount(provider.connection, tokenAccount);
    expect(Number(userAta.amount)).to.equal(10*10^6);
  });

  it("Movie review is updated", async () => {
    const newDescription = "wow, this is new";
    const newRating = 4;

    const tx = await program.methods
    .updateMovieReview(order.marketId, order.description, order.contracts)
    .accounts({
      movieReview: order_pda
    })
    .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.orderAccountState.fetch(order_pda);
    expect(order.marketId === account.marketId);
    expect(newDescription === account.description);
    expect(newRating === account.contracts);
    expect(provider.wallet.publicKey === account.trader);
  });

  it("Movie review is deleted", async() => {
    const tx = await program.methods
      .deleteMovieReview(order.marketId)
      .accounts({ movieReview: order_pda })
      .rpc();
  });
});