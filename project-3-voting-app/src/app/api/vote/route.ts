import {ActionGetResponse, ActionPostRequest, ACTIONS_CORS_HEADERS, createPostResponse} from "@solana/actions";
import {Connection, PublicKey, Transaction} from "@solana/web3.js";
import {Voting} from "@/../anchor/target/types/voting";
import { BN, Program } from "@coral-xyz/anchor";

export const OPTIONS = GET;
const idl = require("@/../anchor/target/idl/voting.json");

export async function GET(request: Request) {
  const actionMetadata: ActionGetResponse = {
    icon: "https://plus.unsplash.com/premium_photo-1726072356924-e29e8999df09",
    title: "Vote for your favorite type of peanut butter!",
    description: "Vote between crunchy and smooth peanut butter",
    label: "Vote",
    links: {
      actions: [{
        label: "Vote for Crunchy",
        href: "/api/vote?candidate=Crunchy",
        type: "transaction"
      }, {
        label: "Vote for Smooth",
        href: "/api/vote?candidate=Smooth",
        type: "transaction"
      }]
    }
  };

  return Response.json(actionMetadata, {
    headers: ACTIONS_CORS_HEADERS
  });
}

export async function POST(request: Request) {
  const url = new URL(request.url);
  let candidate = url.searchParams.get("candidate");

  if (!candidate || !["Crunchy", "Smooth"].includes(candidate)) {
    return new Response("Invalid candidate", { status: 400, headers: ACTIONS_CORS_HEADERS });
  }

  const connection = new Connection("http://127.0.01:8899", "confirmed");
  const program: Program<Voting> = new Program(idl, { connection });
  const body: ActionPostRequest = await request.json();

  let signer;
  try {
    signer = new PublicKey(body.account);
  } catch (e) {
    return new Response("Invalid account", { status: 400, headers: ACTIONS_CORS_HEADERS})
  }

  const instructions = await program.methods.vote(new BN(1), candidate)
    .accounts({ signer })
    .instruction();
  const blockhash = await connection.getLatestBlockhash();

  const transaction = new Transaction({
      blockhash: blockhash.blockhash,
      feePayer: signer,
      lastValidBlockHeight: blockhash.lastValidBlockHeight
    })
    .add(instructions);

  const response = await createPostResponse({
    fields: {
      type: 'transaction',
      transaction,
    }
  });

  return Response.json(response, { headers: ACTIONS_CORS_HEADERS });
}