import idl from '../idl/solana_crowdfund.json';
import { Program, web3 } from '@project-serum/anchor';
const { PublicKey } = web3;

export const PROGRAM_ID = new PublicKey('REPLACE_WITH_PROGRAM_ID');
export const getProgram = (provider) => new Program(idl, PROGRAM_ID, provider);
