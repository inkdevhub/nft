import { useEffect, useState } from 'react';
import BN from 'bn.js';
import { ApiPromise, WsProvider } from '@polkadot/api';
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { ContractPromise } from '@polkadot/api-contract';
import { web3FromSource } from '@polkadot/extension-dapp';

import axios from 'axios';
import Header from './Header';
import Footer from './Footer';
import SampleContractsList from './SampleContractsList';

// Specify the metadata of the contract.
import abi from '../metadata/metadata_sample.json';
//import abi from '../metadata/metadata_sample_1108.json';

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
    {
      name: 'Custom',
      url: '',
    },
  ];

  const [block, setBlock] = useState(0);
  const [blockchainUrl, setBlockchainUrl] = useState('');
  const [blockchainName, setBlockchainName] = useState('');
  const [actingChainName, setActingChainName] = useState('');
  const [actingChainUrl, setActingChainUrl] = useState('');
  const [customUrl, setCustomUrl] = useState('');

  const [accounts, setAccounts] = useState<InjectedAccountWithMeta[]>([]);
  const [actingAddress, setActingAddress] = useState('');
  const [api, setApi] = useState<any>();
  
  const [contractAddress, setContractAddress] = useState('');
  const [tokenId, setTokenId] = useState('');
  const [viewContractAddress, setViewContractAddress] = useState('');
  const [viewTokenId, setViewTokenId] = useState('');
  const [tokenURI, setTokenURI] = useState('');
  const [ownerAddress, setOwnerAddress] = useState('');

  const [result, setResult] = useState('');
  const [gasConsumed, setGasConsumed] = useState("");
  const [outcome, setOutcome] = useState('');
  
  const [tokenImageURI, setTokenImageURI] = useState('');
  const [ipfsImageURI, setIpfsImageURI] = useState('');
  const [tokenName, setTokenName] = useState('');
  const [tokenDescription, setTokenDescription] = useState('');
  const [subScanUri, setSubScanUri] = useState('');
  const [subScanTitle, setSubScanTitle] = useState('');
  const [totalSupply, setTotalSupply] = useState('');
  const [maxSupply, setMaxSupply] = useState('');

  const [ipfsGateway, setIpfsGateway] = useState('');
  const [actingIpfsGateway, setActingIpfsGateway] = useState('Pinata');

  useEffect(() => {
    setIpfsGateway(actingIpfsGateway);
    const url:any = localStorage.getItem('customUrl');
    setCustomUrl(url);
  },[]);
  
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
    const value = new BN(10).pow(new BN(18));
    //const value = (new BN(10).pow(new BN(19)));
    //dm = new BN(balance).divmod(base);
    //return parseFloat(dm.div.toString() + "." + dm.mod.toString())

    const contract = new ContractPromise(api, abi, contractAddress);
    const account = accounts.filter(data => data.address === actingAddress);

    const mintTokenExtrinsic =
      await contract.tx['shiden34Trait::mintNext']({value: value, gasLimit: gasLimit});
//      await contract.tx['psp34Custom::mintNext']({value: value, gasLimit: gasLimit});

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
        if (actingChainName === 'Local') {
          window.setTimeout(function(){
            getTotalSupply(contractAddress);
          }, 2000);
        }
      } else if (status.isFinalized) {
        console.log('finalized');
        setGasConsumed("finalized");
        if (actingChainName !== 'Local') {
          getTotalSupply(contractAddress);
        }
      } else {
        console.log(`Current status: ${status.type}`);
        setGasConsumed("Current status: " + status.type.toString());
      }
    }).catch((error: any) => {
      console.log(':( transaction failed', error);
      setGasConsumed(":( transaction failed: " + error.toString());
    });

  };

  async function getTokenURI(address: string, tokenIdstr: string) {
    if (!blockchainUrl || !block) {
      alert('Please select Blockchain and click "Set Blockchain" button.');
      return;
    }
    let isExecGetTotalSupply = false;
    if (address === '' && tokenIdstr === '') {
      address = viewContractAddress;
      tokenIdstr = viewTokenId;
      isExecGetTotalSupply = true;
      setTotalSupply('');
      setOwnerAddress('');
    }

    setTokenURI('');
    setTokenImageURI('');
    setIpfsImageURI('');
    setTokenName('');
    setTokenDescription('');
    setMaxSupply('');

    const contract = new ContractPromise(api, abi, address);
    const {result, output} = 
    await contract.query['shiden34Trait::tokenUri'](
//    await contract.query['psp34Custom::tokenUri'](
        address,
        {value: 0, gasLimit: -1},
        //{u64: viewTokenId});
        {u64: Number(tokenIdstr)});
    
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
          const matadataUrl = getIpfsGatewayUri(url);
          axios.get(matadataUrl).then(res => {
            // TokenURI metadata
            console.log(res.data);
            const imageUrl = res.data.image.toString();
            let metadataImageUrl = getIpfsGatewayUri(imageUrl);
            setTokenImageURI(imageUrl);
            setIpfsImageURI(metadataImageUrl);
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

        setViewTokenId(tokenIdstr);
        getMaxSupply(address);
        if (isExecGetTotalSupply) {
          getTotalSupply('');
        }
        window.setTimeout(function(){
          getOwnerOf(address, tokenIdstr);
        }, 3000);

      } else {
        setOutcome(outputData.toString());
        setTokenURI('');
        setTokenImageURI('');
        setIpfsImageURI('');
        setTokenName('');
        setTokenDescription('');
        setMaxSupply('');
        setTotalSupply('');
        setOwnerAddress('');
      }

    } else {
      setOutcome('');
      setTokenURI('');
      setTokenImageURI('');
      setIpfsImageURI('');
      setTokenName('');
      setTokenDescription('');
      setMaxSupply('');
      setTotalSupply('');
      setOwnerAddress('');
    }
  };

  async function getOwnerOf(address: string, tokenIdstr: string) {
    if (address === '' && tokenIdstr === '') {
      address = viewContractAddress;
      tokenIdstr = viewTokenId;
    }
    const contract = new ContractPromise(api, abi, address);
    const {result, output} = 
      await contract.query['psp34::ownerOf'](
        address,
        {value: 0, gasLimit: -1},
        {u64: tokenIdstr});
    
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

  async function getMaxSupply(address: string) {
    const contract = new ContractPromise(api, abi, address);
    const {result, output} = 
      await contract.query['shiden34Trait::maxSupply'](
        address,
        {value: 0, gasLimit: -1});
    
    // The actual result from RPC as `ContractExecResult`
    console.log(result.toHuman());

    // check if the call was successful
    if (result.isOk) {
      // output the return value
      console.log('Success', output?.toHuman());
      const outcome: any = output;
      const resultStr: string = outcome.toHuman()?.toString()!; 
      if (resultStr) {
        setMaxSupply(resultStr);
console.log('setMaxSupply: ', resultStr);
      } else {
        setMaxSupply('0');
      }
    }
  };

  async function getTotalSupply(address: string) {
    let isExecGetTokenURI:boolean = true;
    if (address === '') {
      address = viewContractAddress;
      isExecGetTokenURI = false;
    }
    const contract = new ContractPromise(api, abi, address);
    const {result, output} = 
      await contract.query['psp34::totalSupply'](
        address,
        {value: 0, gasLimit: -1});
    
    // The actual result from RPC as `ContractExecResult`
    console.log(result.toHuman());

    // check if the call was successful
    if (result.isOk) {
      // output the return value
      console.log('Success', output?.toHuman());
      const outcome: any = output;
      const resultStr: string = outcome.toHuman()?.toString()!; 
      if (resultStr) {
        setTotalSupply(resultStr);
console.log('setTotalSupply: ', resultStr);
        setViewContractAddress(address);
        if (isExecGetTokenURI) {
          getTokenURI(address, resultStr);
        }
      } else {
        setTotalSupply('0');
      }
    }
  };

  const setup = async () => {
 
    const newDataset = blockchains.filter(data => data.name === blockchainName);

    let chainUrl = '';
    if (blockchainName === 'Custom') {
      chainUrl = customUrl;
    } else {
      chainUrl = newDataset[0]?.url;
    }
    setBlockchainUrl(chainUrl);

    if (!chainUrl) {
      return;
    }

    setActingChainName('');
    setBlock(0);
    setActingChainUrl('');

    const wsProvider = new WsProvider(chainUrl);
    const api = await ApiPromise.create({provider: wsProvider});
    const unsubscribe = await api.rpc.chain.subscribeNewHeads((lastHeader) => {
      setApi(api);
      setActingChainName(blockchainName);
      setBlock(lastHeader.number.toNumber());
      setActingChainUrl(chainUrl);
      unsubscribe();
    });
  };

  const saveIpfsGateway = () => {
    localStorage.setItem("ipfsGateway", ipfsGateway);
    setActingIpfsGateway(ipfsGateway);
  };

  const saveCustomURL = (url: string) => {
    setCustomUrl(url);
    localStorage.setItem('customUrl', url);
  };

  const getIpfsGatewayUri = (uri: string) => {
    if (!uri) {
      return '';
    }

    const scheme = uri.slice(0, 7);
    let cid = '';
    let fileName = '';
    if (scheme === 'ipfs://') {
      let tmp = uri.substr(7);
      cid = uri.substr(7, tmp.indexOf('/'));
      fileName = uri.substr(tmp.indexOf('/') + 8);
      if (actingIpfsGateway === 'ipfs.io') {
        uri = 'https://ipfs.io/ipfs/' + cid + '/' + fileName;
      } else if (actingIpfsGateway === 'Crust Network') {
        uri = 'https://gw.crustapps.net/ipfs/' + cid + '/' + fileName;
      } else if (actingIpfsGateway === 'Cloudflare') {
        uri = 'https://cloudflare-ipfs.com/ipfs/' + cid + '/' + fileName;
      } else if (actingIpfsGateway === 'dweb.link') {
        uri = 'https://dweb.link/ipfs/' + cid + '/' + fileName;
        //cid = cid.toV1().toString('base32');
        //uri = 'https://' + cid + '.ipfs.dweb.link' + '/' + fileName;
      } else if (actingIpfsGateway === 'Pinata') {
        uri = 'https://cielo.mypinata.cloud/ipfs/' + cid + '/' + fileName;
      }
    }
    console.log('ipfs_uri: ', uri);
    return uri;
  };

  return (
    <div className="text-center">
      <Header />
      <div className="p-3 mt-2 m-auto max-w-6xl w-11/12 border-[#d8d2c5] dark:border-[#323943] bg-[#f4efe2] dark:bg-[#121923] border border-1 rounded">
        <div className="mb-5 text-xl">Select blockchain</div>
        <button
          className="bg-[#184e9b] hover:bg-[#2974df] hover:duration-500 text-white rounded px-4 py-2"
          onClick={setup}
        >Set Blockchain</button>
        <select
          className="p-3 m-3 mt-0 mb-2 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
          onChange={(event) => {
            setBlockchainName((event.target.value));
          }}
        >
            <option value="None">-- select --</option>
            <option value="Shiden">Shiden</option>
            <option value="Shibuya">Shibuya</option>
            <option value="Local">Local</option>
            <option value="Custom">Custom</option>
        </select>
        {blockchainName === 'Custom' ?
        <input
            className="p-2 m-2 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
            onChange={(event) => saveCustomURL(event.target.value)}
            placeholder="Custom URL"
            value={customUrl}
          />
        : <></> }
        <div className="m-2">Current Blockchain: {actingChainName? actingChainName : "---"} / {actingChainUrl? actingChainUrl : "---"}</div>
      </div>

      <div className="text-left p-2 pt-0 mt-5 m-auto max-w-6xl w-11/12 border-[#d8d2c5] dark:border-[#323943] bg-[#f4efe2] dark:bg-[#121923] border border-1 rounded">
        <div className="text-center mt-4">
          <div className="mb-2 text-xl">Connect wallet</div>
          <button
              className="bg-[#184e9b] hover:bg-[#2974df] hover:duration-500 disabled:bg-[#a4a095] dark:disabled:bg-stone-700 text-white rounded px-4 py-2"
              onClick={extensionSetup}
            >
              Set Account
          </button>
          <select
          className="p-3 m-3 mt-0 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
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
      </div>

      <div className="text-left p-2 pt-0 mt-5 m-auto max-w-6xl w-11/12 border-[#d8d2c5] dark:border-[#323943] bg-[#f4efe2] dark:bg-[#121923] border border-1 rounded">
        <div className="text-center mt-4">
          <div className="mb-2 text-xl">Mint NFT</div>
          <button disabled={!contractAddress}
            className="bg-[#184e9b] hover:bg-[#2974df] hover:duration-500 disabled:bg-[#a4a095] dark:disabled:bg-stone-700 text-white rounded px-4 py-2"
            onClick={execMint}
          >{contractAddress ? 'Mint NFT' : 'Enter ContractAddress'}</button>
          <input
            className="p-2 m-2 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
            onChange={(event) => setContractAddress(event.target.value.trim())}
            placeholder="ContractAddress"
          /> (Mint cost: 1.00)
          <p className="p-1 m-1 break-all">Status: {gasConsumed}</p>
        </div>
      </div>

      <div className="text-left p-2 pt-0 mt-5 m-auto max-w-6xl w-11/12 border-[#d8d2c5] dark:border-[#323943] bg-[#f4efe2] dark:bg-[#121923] border border-1 rounded">
        <div className="text-center mt-4">
          <div className="mb-3 text-xl">NFT View</div>
          <button disabled={!viewContractAddress || !viewTokenId}
            className="bg-[#184e9b] hover:bg-[#2974df] hover:duration-500 disabled:bg-[#a4a095] dark:disabled:bg-stone-700 text-white rounded px-4 py-2"
            onClick={(event) => getTokenURI('', '')}
          >{viewContractAddress || viewTokenId ? 'View NFT' : 'Enter Blank Form'}</button>
          <input
            className="p-2 m-2 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
            onChange={(event) => setViewContractAddress(event.target.value.trim())}
            placeholder="ContractAddress"
            value={viewContractAddress}
          />
          <input
            className="p-2 m-2 w-20 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
            onChange={(event) => setViewTokenId(event.target.value.trim())}
            placeholder="TokenID"
            value={viewTokenId}
          />
        </div>

        <div className="text-center">
          <div>
            <div className='h-64 flex justify-center items-center'>
              {tokenURI ? ipfsImageURI ?
                <img className="p-2 m-auto h-64 duration-300" src={ipfsImageURI} /> :
                <img className="h-32 duration-300" src="./loading_default.svg" /> :
                <img className="h-32 duration-300" src="./image-placeholder.png" />
              }
            </div>
            <p className="p-1 m-1 text-xl break-words">{tokenName}</p>
            <p className="p-1 m-1 break-words">{tokenDescription}</p>
            <p className={contractAddress ? "m-1 break-all" : "hidden"}><a className="hover:text-gray-400" target="_blank" rel="noreferrer" href={subScanUri}>{subScanTitle}</a></p>
          </div>
        </div>

        <div className="m-2 mt-4 p-2 bg-[#dcd6c8] dark:bg-[#020913] rounded">
          <p className="p-1 m-1 break-all">Result: {result}</p>
          <p className="p-1 m-1 break-all">OutputData: {outcome}</p>
          <p className="p-1 m-1">TokenId: {viewTokenId}</p>
          <p className="p-1 m-1 break-all">TokenURI: {tokenURI}</p>
          <p className="p-1 m-1 break-all" >ImageURI: {tokenImageURI}</p>
          <p className="p-1 m-1 break-all" >MaxSupply: {maxSupply}</p>
          <p className="p-1 m-1 break-all" >totalSupply: {totalSupply}</p>
          <p className="p-1 m-1 break-all">OwnerAddress: {ownerAddress}</p>
        </div>
      </div>

      <div className="p-3 mt-5 m-auto max-w-6xl w-11/12 border-[#d8d2c5] dark:border-[#323943] bg-[#f4efe2] dark:bg-[#121923] border border-1 rounded">
        <div className="mb-5 text-xl">Select ipfs Gateway</div>
        <button
          className="mb-2 bg-[#184e9b] hover:bg-[#2974df] hover:duration-500 text-white rounded px-4 py-2"
          onClick={saveIpfsGateway}
        >
          Set ipfs Gateway
        </button>
        <select
          className="p-3 m-3 mt-0 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 dark:border-gray-300 rounded"
          onChange={(event) => {
            setIpfsGateway((event.target.value));
          }}
        >
            <option value="Pinata">Pinata</option>
            <option value="Cloudflare">Cloudflare</option>
            <option value="Crust Network">Crust Network</option>
            <option value="ipfs.io">ipfs.io</option>
            <option value="dweb.link">dweb.link</option>
        </select>
        <div className="p-2 m-2 mt-0">Current ipfs Gateway: {actingIpfsGateway? actingIpfsGateway : "---"}</div>
      </div>

      <SampleContractsList />
      <Footer />
    </div>
  );
};

export default IndexCanvas;