import { useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { getProvider } from '../utils/connection';
import { getProgram } from '../utils/program';
import { web3, BN } from '@project-serum/anchor';

export default function CampaignActions() {
  const wallet = useWallet();
  const [goal, setGoal] = useState('');
  const [deadline, setDeadline] = useState('');
  const [amount, setAmount] = useState('');

  const initCampaign = async () => {
    if (!wallet.publicKey) return;
    const provider = getProvider(wallet);
    const program = getProgram(provider);
    const goalLamports = new BN(parseFloat(goal) * web3.LAMPORTS_PER_SOL);
    const deadlineTs = new BN(Math.floor(Date.now() / 1000 + parseInt(deadline) * 60));
    await program.methods
      .initialize(goalLamports, deadlineTs)
      .accounts({ creator: wallet.publicKey })
      .rpc();
  };

  const contribute = async () => {
    if (!wallet.publicKey) return;
    const provider = getProvider(wallet);
    const program = getProgram(provider);
    const amountLamports = new BN(parseFloat(amount) * web3.LAMPORTS_PER_SOL);
    await program.methods
      .contribute(amountLamports)
      .accounts({ contributor: wallet.publicKey })
      .rpc();
  };

  const withdraw = async () => {
    if (!wallet.publicKey) return;
    const provider = getProvider(wallet);
    const program = getProgram(provider);
    await program.methods
      .withdraw()
      .accounts({ creator: wallet.publicKey })
      .rpc();
  };

  const refund = async () => {
    if (!wallet.publicKey) return;
    const provider = getProvider(wallet);
    const program = getProgram(provider);
    await program.methods
      .refund()
      .accounts({ contributor: wallet.publicKey })
      .rpc();
  };

  return (
    <div className="space-y-8">
      <section>
        <h2 className="text-lg font-semibold mb-2">Init Campaign</h2>
        <div className="flex flex-col sm:flex-row gap-2">
          <input
            type="number"
            placeholder="Goal (SOL)"
            value={goal}
            onChange={(e) => setGoal(e.target.value)}
            className="p-2 rounded border border-gray-300"
          />
          <input
            type="number"
            placeholder="Deadline (min)"
            value={deadline}
            onChange={(e) => setDeadline(e.target.value)}
            className="p-2 rounded border border-gray-300"
          />
          <button onClick={initCampaign} className="bg-blue-500 text-white px-4 py-2 rounded">Init</button>
        </div>
      </section>

      <section>
        <h2 className="text-lg font-semibold mb-2">Contribute</h2>
        <div className="flex flex-col sm:flex-row gap-2">
          <input
            type="number"
            placeholder="Amount (SOL)"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            className="p-2 rounded border border-gray-300"
          />
          <button onClick={contribute} className="bg-green-500 text-white px-4 py-2 rounded">Contribute</button>
        </div>
      </section>

      <section className="flex gap-4">
        <button onClick={withdraw} className="bg-yellow-500 text-white px-4 py-2 rounded">Withdraw</button>
        <button onClick={refund} className="bg-red-500 text-white px-4 py-2 rounded">Refund</button>
      </section>
    </div>
  );
}
