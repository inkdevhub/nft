import { useEffect, useState } from 'react';
import BN from 'bn.js';
import { ApiPromise, WsProvider } from '@polkadot/api';
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { ContractPromise } from '@polkadot/api-contract';
import axios from 'axios';
import Header from './Header';
import Footer from './Footer';
import SampleContractsList from './SampleContractsList';

import {
  web3FromSource
} from '@polkadot/extension-dapp';


// Specify the metadata of the contract.
import abi from '../metadata/metadata_sample.json';

const IndexCanvas = () => {
  
  const blockchains = [
    {
      name: 'Shiden', 
      url: 'wss://shiden.api.onfinality.io/public-ws',
      subscan_url: 'https://shiden.subscan.io/account/',
    },
    {
      name: 'Shibuya',
      url: 'wss://rpc.shibuya.astar.network',
      subscan_url: 'https://shibuya.subscan.io/account/',
    },
    {
      name: 'Local',
      url: 'ws://127.0.0.1:9944',
    },
  ];

  const [block, setBlock] = useState(0);
  const [lastBlockHash, setLastBlockHash] = useState('');
  const [blockchainUrl, setBlockchainUrl] = useState('');
  const [blockchainName, setBlockchainName] = useState('');
  const [actingChainName, setActingChainName] = useState('');
  const [actingChainUrl, setActingChainUrl] = useState('');

  const [accounts, setAccounts] = useState<InjectedAccountWithMeta[]>([]);
  const [actingAddress, setActingAddress] = useState("");
  const [api, setApi] = useState<any>();

  const [contractAddress, setContractAddress] = useState('');
  const [tokenId, setTokenId] = useState('');
  const [tokenURI, setTokenURI] = useState('');
  const [ownerAddress, setOwnerAddress] = useState('');
  
  const [result, setResult] = useState('');
  const [gasConsumed, setGasConsumed] = useState("");
  const [outcome, setOutcome] = useState('');
  
  const [tokenImageURI, setTokenImageURI] = useState('');
  const [tokenName, setTokenName] = useState('');
  const [tokenDescription, setTokenDescription] = useState('');
  const [subScanUri, setSubScanUri] = useState('');
  const [subScanTitle, setSubScanTitle] = useState('');

  useEffect(() => {
  });
  
  const extensionSetup = async () => {
    if (!blockchainUrl || !block) {
      alert("Please select Blockchain and click 'Set Blockchain' button.");
      return;
    }
    const { web3Accounts, web3Enable } = await import(
      "@polkadot/extension-dapp"
    );
    const extensions = await web3Enable("Showcase PSP34 Mint Sample");
    if (extensions.length === 0) {
      return;
    }
    const account = await web3Accounts();
    setAccounts(account);
    if (!actingAddress) {
      setActingAddress(account[0].address);
    }
  };

  async function execMint() {
    if (!blockchainUrl || !block || accounts.length == 0) {
      alert("Please select Blockchain and click 'Set Blockchain' button and click 'Set Account' button.");
      return;
    }
    const gasLimit = 30000 * 1000000;
    const value = 1000000000000000; //new BN(1);

    const contract = new ContractPromise(api, abi, contractAddress);
    const account = accounts.filter(data => data.address === actingAddress);

    const mintTokenExtrinsic =
      await contract.tx['shiden34Trait::mintNext']({value: value, gasLimit: gasLimit});

    let injector: any;
    if (accounts.length == 1) {
      injector = await web3FromSource(accounts[0].meta.source);
    } else if (accounts.length > 1) {
      injector = await web3FromSource(account[0].meta.source);
    } else {
      return;
    }

    mintTokenExtrinsic.signAndSend(actingAddress, { signer: injector.signer }, ({ status }) => {
      if (status.isInBlock) {
        console.log(`Completed at block hash #${status.asInBlock.toString()}`);
        setGasConsumed("Completed at block hash #" + status.asInBlock.toString());
      } else if (status.isFinalized) {
        console.log('finalized');
        setGasConsumed("finalized");
      } else {
        console.log(`Current status: ${status.type}`);
        setGasConsumed("Current status: " + status.type.toString());
      }
    }).catch((error: any) => {
      console.log(':( transaction failed', error);
      setGasConsumed(":( transaction failed: " + error.toString());
    });

  };

  async function getTokenURI() {
    if (!blockchainUrl || !block) {
      alert('Please select Blockchain and click "Set Blockchain" button.');
      return;
    }
    const contract = new ContractPromise(api, abi, contractAddress);
    const {result, output} = 
      await contract.query['shiden34Trait::tokenUri'](
        contractAddress,
        {value: 0, gasLimit: -1},
        {u64: tokenId});
    
    setResult(JSON.stringify(result.toHuman()));

    // The actual result from RPC as `ContractExecResult`
    console.log(result.toHuman());

    // check if the call was successful
    if (result.isOk) {
      // output the return value
      console.log('Success', output?.toHuman());
      const outputData: any = output;
      setOutcome(outputData.toString());

      if (outputData.isOk) {
        const url = outputData.inner.toString();
        if (url !== undefined) {
          setTokenURI(url);
          axios.get(url).then(res => {
            // TokenURI metadata
            console.log(res.data);
            setTokenImageURI(res.data.image.toString());
            setTokenName(res.data.name.toString());
            setTokenDescription(res.data.description.toString());
          });
        }
        
        if (actingChainName === 'Shiden' || actingChainName === 'Shibuya') {
          const newDataset = blockchains.filter(data => data.name === actingChainName);
          const subScanBaseUri = newDataset[0]?.subscan_url;
          setSubScanUri(subScanBaseUri + contractAddress);
          setSubScanTitle('Show on Subscan (' + actingChainName + ')');
        } else {
          setSubScanTitle('');
        }
    
        getOwnerOf();

      } else {
        setOutcome(outputData.toString());
        setTokenURI('');
        setTokenImageURI('');
        setTokenName('');
        setTokenDescription('');
        setOwnerAddress('');
      }

    } else {
      setOutcome('');
      setTokenURI('');
      setTokenImageURI('');
      setTokenName('');
      setTokenDescription('');
      setOwnerAddress('');
    }
  };

  async function getOwnerOf() {
    const contract = new ContractPromise(api, abi, contractAddress);
    const {result, output} = 
      await contract.query['psp34::ownerOf'](
        contractAddress,
        {value: 0, gasLimit: -1},
        {u64: tokenId});
    
    // The actual result from RPC as `ContractExecResult`
    console.log(result.toHuman());

    // check if the call was successful
    if (result.isOk) {
      // output the return value
      console.log('Success', output?.toHuman());
      const outcome: any = output;
      const resultStr: string = outcome.toHuman()?.toString()!; 
      if (resultStr) {
        setOwnerAddress(resultStr);
      } else {
        setOwnerAddress('none');
      }
    }
  };

  const setup = async () => {

    const newDataset = blockchains
      .filter(data => data.name === blockchainName);
    const chainUrl = newDataset[0]?.url;
    setBlockchainUrl(newDataset[0]?.url);

    if (!chainUrl) {
      return;
    }

    const wsProvider = new WsProvider(chainUrl);
    const api = await ApiPromise.create({provider: wsProvider});
    await api.rpc.chain.subscribeNewHeads((lastHeader) => {
      setApi(api);
      setActingChainName(blockchainName);
      setBlock(lastHeader.number.toNumber());
      setLastBlockHash(lastHeader.hash.toString());
      setActingChainUrl(chainUrl);
      //console.log(api.hasSubscriptions);
    });
  };

  return (
    <div className="text-center">
      <Header />
      <div className="p-3 mt-2 m-auto max-w-6xl w-11/12 border-[#323943] bg-[#121923] border border-1 rounded">
        <div className="mb-5 text-xl">Select blockchain</div>
        <button
          className="bg-[#184e9b] hover:bg-[#1964cf] hover:duration-500 text-white rounded px-4 py-2"
          onClick={setup}
        >
          Set Blockchain
        </button>
        <select
          className="p-3 m-3 mt-0 bg-[#020913] border-2 border-gray-300 rounded"
          onChange={(event) => {
            setBlockchainName((event.target.value));
          }}
        >
            <option value="None">-- select --</option>
            <option value="Shiden">Shiden</option>
            <option value="Shibuya">Shibuya</option>
            <option value="Local">Local</option>
        </select>

        <div className="p-2 m-2 mt-0">Current Blockchain Name: {actingChainName? actingChainName : "---"}</div>
        <div className="p-2 m-2 mt-0">Current Blockchain URL: {actingChainUrl? actingChainUrl : "---"}</div>
        <div className="p-1 m-1">Block: {block? block : "---"}</div>
        <div className="p-1 m-auto w-11/12 break-all">Last block hash: {lastBlockHash? lastBlockHash : "---"}</div>
      </div>

      <div className="text-center p-2 pt-0 mt-5 m-auto max-w-6xl w-11/12 border-[#323943] bg-[#121923] border border-1 rounded">
        <button
            className="bg-[#184e9b] hover:bg-[#1964cf] hover:duration-500 disabled:bg-stone-700 text-white rounded px-4 py-2"
            onClick={extensionSetup}
          >
            Set Account
        </button>
        <select
          className="p-3 m-3 bg-[#020913] border-2 border-gray-300 rounded"
          onChange={(event) => {
            console.log(event);
            setActingAddress(event.target.value);
          }}
        >
          {accounts.map((a) => (
            <option key={a.address} value={a.address}>
              [{a.meta.name}]
            </option>
          ))}
        </select>
        <p className="p-1 m-1 break-all">actingAddress: {actingAddress}</p>
      </div>

      <div className="text-center p-2 pt-0 mt-5 m-auto max-w-6xl w-11/12 border-[#323943] bg-[#121923] border border-1 rounded">
        <button disabled={!contractAddress}
          className="bg-[#184e9b] hover:bg-[#1964cf] hover:duration-500 disabled:bg-stone-700 text-white rounded px-4 py-2"
          onClick={execMint}
        >{contractAddress ? 'Mint NFT' : 'Enter ContractAddress'}</button>
        <input
          className="p-2 m-2 bg-[#020913] border-2 border-gray-300 rounded"
          onChange={(event) => setContractAddress(event.target.value)}
          placeholder="ContractAddress"
        /> (Mint cost: 0.001)
        <p className="p-1 m-1 break-all">Status: {gasConsumed}</p>
      </div>

      <div className="text-left p-2 pt-0 mt-5 m-auto max-w-6xl w-11/12 border-[#323943] bg-[#121923] border border-1 rounded">
        <div className="text-center mt-4">
          <div className="mb-3 text-xl">NFT View</div>
          <button disabled={!contractAddress || !tokenId}
            className="bg-[#184e9b] hover:bg-[#1964cf] hover:duration-500 disabled:bg-stone-700 text-white rounded px-4 py-2"
            onClick={getTokenURI}
          >{contractAddress || tokenId ? 'View NFT' : 'Enter Blank Form'}</button>
          <input
            className="p-2 m-2 bg-[#020913] border-2 border-gray-300 rounded"
            onChange={(event) => setContractAddress(event.target.value)}
            placeholder="ContractAddress"
          />
          <input
            className="p-2 m-2 w-20 bg-[#020913] border-2 border-gray-300 rounded"
            onChange={(event) => setTokenId(event.target.value)}
            placeholder="TokenID"
          />
        </div>

        <div className="text-center">
          <div>
            <img className="p-2 m-auto w-64" src={tokenImageURI} />
            <p className="p-1 m-1 text-xl break-words">{tokenName}</p>
            <p className="p-1 m-1 break-words">{tokenDescription}</p>
            <p className={contractAddress ? "m-1 break-all" : "hidden"}><a className="hover:text-gray-400" target="_blank" rel="noreferrer" href={subScanUri}>{subScanTitle}</a></p>
          </div>
        </div>

        <div className="m-2 mt-4 p-2 bg-[#020913] rounded">
          <p className="p-1 m-1 break-all">Result: {result}</p>
          <p className="p-1 m-1 break-all">OutputData: {outcome}</p>
          <p className="p-1 m-1">TokenId: {tokenId}</p>
          <p className="p-1 m-1 break-all">TokenURI: {tokenURI}</p>
          <p className="p-1 m-1 break-all" >ImageURI: {tokenImageURI}</p>
          <p className="p-1 m-1 break-all">OwnerAddress: {ownerAddress}</p>
        </div>
      </div>
      <SampleContractsList />
      <Footer />
    </div>
  );
};

export default IndexCanvas;