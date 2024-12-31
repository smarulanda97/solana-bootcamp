import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Voting } from '../target/types/voting';
import {BankrunProvider, startAnchor} from "anchor-bankrun";
import {PublicKey} from "@solana/web3.js";
import {Buffer} from "buffer";
import * as console from "node:console";

const idl = require("../target/idl/voting.json");
const votingProgramId = new PublicKey("74dCTDkTRbZTogHjgCDSQNn59JQBgd2GwkzUCaZqGHW6");

describe('voting', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  let context;
  let provider;
  let votingProgram: Program<Voting>;

  beforeAll(async () => {
    context = await startAnchor("", [{ name: "voting", programId: votingProgramId }], []);
    provider = new BankrunProvider(context);
    votingProgram = new Program<Voting>(idl, provider);
  })

  it('should initialize the poll', async () => {
    const tx = await votingProgram.methods.initializePoll(
        new anchor.BN(1),
        "What is your favorite football team?",
        new anchor.BN(0),
        new anchor.BN(1798672339),
      )
      .rpc();

    const [pollAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, 'le', 8)],
      votingProgramId
    );

    const poll_account = await votingProgram.account.pollAccount.fetch(pollAddress);

    expect(poll_account.id.toNumber()).toEqual(1);
    expect(poll_account.description).toEqual("What is your favorite football team?");
    expect(poll_account.startTime.toNumber()). toBeLessThan(poll_account.endTime.toNumber());
  });

  it('should initialize the candidate', async () => {
    await votingProgram.methods.initializeCandidate(new anchor.BN(1), "Smooth")
      .rpc();

    await votingProgram.methods.initializeCandidate(new anchor.BN(1), "Crunchy")
      .rpc()

    const [smoothAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, 'le', 8), Buffer.from("Smooth")],
      votingProgramId
    );
    const smoothCandidate = await votingProgram.account.candidateAccount.fetch(smoothAddress);

    const [crunchyAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, 'le', 8), Buffer.from("Crunchy")],
      votingProgramId
    );
    const crunchyCandidate = await votingProgram.account.candidateAccount.fetch(crunchyAddress);

    expect(smoothCandidate.name).toEqual("Smooth");
    expect(smoothCandidate.votes.toNumber()).toEqual(0);
    expect(crunchyCandidate.name).toEqual("Crunchy");
    expect(crunchyCandidate.votes.toNumber()).toEqual(0);
  })

  it('should vote', async () => {
    await votingProgram.methods.vote(new anchor.BN(1), "Crunchy")
      .rpc();

    const [crunchyAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, 'le', 8), Buffer.from("Crunchy")],
      votingProgramId
    );
    const crunchyCandidate = await votingProgram.account.candidateAccount.fetch(crunchyAddress);

    expect(crunchyCandidate.votes.toNumber()).toEqual(1);
  })
});
