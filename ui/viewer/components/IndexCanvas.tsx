import { useEffect, useState } from 'react';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { ContractPromise } from '@polkadot/api-contract';
import axios from 'axios';
import Header from './Header';
import Footer from './Footer';
import SampleContractsList from './SampleContractsList';

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
    {
      name: 'Custom',
      url: '',
      //url: 'wss://astar-collator.cielo.works:11443',
    },
  ];

  const [block, setBlock] = useState(0);
  const [blockchainUrl, setBlockchainUrl] = useState('');
  const [blockchainName, setBlockchainName] = useState('');
  const [actingChainName, setActingChainName] = useState('');
  const [actingChainUrl, setActingChainUrl] = useState('');
  const [customUrl, setCustomUrl] = useState('');

  const [api, setApi] = useState<any>();
  
  const [contractAddress, setContractAddress] = useState('');
  const [tokenId, setTokenId] = useState('');
  const [tokenURI, setTokenURI] = useState('');
  const [ownerAddress, setOwnerAddress] = useState('');
  
  const [result, setResult] = useState('');
  const [outcome, setOutcome] = useState('');
  
  const [tokenImageURI, setTokenImageURI] = useState('');
  const [ipfsImageURI, setIpfsImageURI] = useState('');
  const [tokenName, setTokenName] = useState('');
  const [tokenDescription, setTokenDescription] = useState('');
  const [subScanUri, setSubScanUri] = useState('');
  const [subScanTitle, setSubScanTitle] = useState('');

  const [ipfsGateway, setIpfsGateway] = useState('');
  const [actingIpfsGateway, setActingIpfsGateway] = useState('Pinata');

  useEffect(() => {
    setIpfsGateway(actingIpfsGateway);
    const url:any = localStorage.getItem('customUrl');
    setCustomUrl(url);
  },[]);

  async function getTokenURI() {
    if (!blockchainUrl || !block) {
      alert('Please select Blockchain and click "Set Blockchain" button.');
      return;
    }

    setTokenURI('');
    setTokenImageURI('');
    setIpfsImageURI('');
    setTokenName('');
    setTokenDescription('');
    setOwnerAddress('');

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
    
        getOwnerOf();

      } else {
        setOutcome(outputData.toString());
        setTokenURI('');
        setTokenImageURI('');
        setIpfsImageURI('');
        setTokenName('');
        setTokenDescription('');
        setOwnerAddress('');
      }

    } else {
      setOutcome('');
      setTokenURI('');
      setTokenImageURI('');
      setIpfsImageURI('');
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
          className="mb-2 bg-[#184e9b] hover:bg-[#2974df] hover:duration-500 text-white rounded px-4 py-2"
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
        <div className="m-2">Current BlockchainName: {actingChainName? actingChainName : "---"}</div>
        <div className="m-2">URL: {actingChainUrl? actingChainUrl : "---"}</div>
      </div>

      <div className="text-left p-2 pt-0 mt-5 m-auto max-w-6xl w-11/12 border-[#d8d2c5] dark:border-[#323943] bg-[#f4efe2] dark:bg-[#121923] border border-1 rounded">
        <div className="text-center mt-4">
          <div className="mb-3 text-xl">NFT View</div>
          <button disabled={!contractAddress || !tokenId}
            className="bg-[#184e9b] hover:bg-[#2974df] hover:duration-500 disabled:bg-[#a4a095] dark:disabled:bg-stone-700 text-white rounded px-4 py-2"
            onClick={getTokenURI}
          >{contractAddress || tokenId ? 'View NFT' : 'Enter Blank Form'}</button>
          <input
            className="p-2 m-2 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
            onChange={(event) => setContractAddress(event.target.value.trim())}
            placeholder="ContractAddress"
          />
          <input
            className="p-2 m-2 w-20 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
            onChange={(event) => setTokenId(event.target.value.trim())}
            placeholder="TokenID"
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
          <p className="p-1 m-1">TokenId: {tokenId}</p>
          <p className="p-1 m-1 break-all">TokenURI: {tokenURI}</p>
          <p className="p-1 m-1 break-all" >ImageURI: {tokenImageURI}</p>
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
          className="p-3 m-3 mt-0 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
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