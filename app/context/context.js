import { createContext, useState, useEffect, useContext, useMemo } from "react";
import { SystemProgram } from "@solana/web3.js";
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import { Keypair } from "@solana/web3.js";

import {
  getProgram,
  getVoterAddress
} from "../utils/program";
import { confirmTx, mockWallet } from "../utils/helper";

export const AppContext = createContext();

export const AppProvider = ({ children }) => {
  const [error, setError] = useState("");
  const [success, setSuccess] = useState("");

  const { connection } = useConnection();
  const wallet = useAnchorWallet();
  const program = useMemo(() => {
    if (connection) {
      return getProgram(connection, wallet ?? mockWallet());
    }
  }, [connection, wallet]);

  useEffect(() => {
    if(votes.length == 0){
      viewVotes();
    }
  }, [program]);

  const [votes, setVotes] = useState([]);

  const viewVotes = async () => {
    const votes = await program.account.voteAccount.all();
    const sortedVotes = votes.sort((a, b) => a.account.votingDeadline - b.account.votingDeadline);
    setVotes(sortedVotes);
  }

  const createVote = async (topic, options, duration) => {
    setError("");
    setSuccess("");
    console.log("Running")
    try {
      const voteAccountKeypair = Keypair.generate();

      const txHash = await program.methods
        .createVote(topic, options, duration)
        .accounts({
          voteAccount: voteAccountKeypair.publicKey,
          user: wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([voteAccountKeypair])
        .rpc();
      await confirmTx(txHash, connection);

      viewVotes();
    } catch (err) {
      console.log("err", err);
      setError(err.message);
    }
  };

  const vote = async (index, votePubKey) => {
    setError("");
    setSuccess("");
    try {
      const voterAccountAddress = await getVoterAddress(votePubKey, wallet.publicKey);
      console.log("voterAccountAddress", voterAccountAddress);
      const txHash = await program.methods
        .vote(index)
        .accounts({
          voteAccount: votePubKey,
          voterAccount: voterAccountAddress,
          user: wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      await confirmTx(txHash, connection);

      viewVotes();
    } catch (err) {
      console.log("err", err);
      setError(err.message);
    }
  };

  return (
    <AppContext.Provider
      value={{
        createVote,
        viewVotes,
        vote,
        votes,
        error,
        success
      }}
    >
      {children}
    </AppContext.Provider>
  );
};

export const useAppContext = () => {
  return useContext(AppContext);
};
