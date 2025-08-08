import WalletConnectButton from './components/WalletConnectButton';
import CampaignActions from './components/CampaignActions';

export default function App() {
  return (
    <div className="min-h-screen bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-gray-100">
      <header className="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
        <h1 className="text-xl font-bold">Solana Crowdfund</h1>
        <WalletConnectButton />
      </header>
      <main className="p-4 max-w-2xl mx-auto">
        <CampaignActions />
      </main>
    </div>
  );
}
