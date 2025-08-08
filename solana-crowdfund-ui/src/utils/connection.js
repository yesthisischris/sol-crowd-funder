import { AnchorProvider } from '@project-serum/anchor';
import { Connection } from '@solana/web3.js';

export const getProvider = (wallet) =>
  new AnchorProvider(
    new Connection('https://api.devnet.solana.com'),
    wallet,
    { preflightCommitment: 'processed' }
  );
