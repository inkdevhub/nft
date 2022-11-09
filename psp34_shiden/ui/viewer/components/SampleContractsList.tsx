import { useEffect, useState } from 'react';

const SampleContractsList = (): JSX.Element => {
  
  const navigation = {
    shiden: [
      { name: 'PSP34Sample', address: 'soon' },
    ],
    shibuya: [
      { name: 'PSP34Sample', address: 'YSXjTTTiqYuUQT51WgMCQspKsw7qiY4Ng2Crp3Mc2hNmATc' },
    ],
  };

  const [shidenContractLabel, setShidenContractLabel] = useState('');
  const [shidenContractAddress, setShidenContractAddress] = useState('');
  const [shibuyaContractLabel, setShibuyaContractLabel] = useState('');
  const [shibuyaContractAddress, setShibuyaContractAddress] = useState('');
  const [localContractLabel, setLocalContractLabel] = useState('');
  const [localContractAddress, setLocalContractAddress] = useState('');
  const [customContractLabel, setCustomContractLabel] = useState('');
  const [customContractAddress, setCustomContractAddress] = useState('');

  useEffect(() => {
    const shidenContractLabel: any = localStorage.getItem('shidenContractLabel');
    setShidenContractLabel(shidenContractLabel);
    const shidenContractAddress: any = localStorage.getItem('shidenContractAddress');
    setShidenContractAddress(shidenContractAddress);
    const shibuyaContractLabel: any = localStorage.getItem('shibuyaContractLabel');
    setShibuyaContractLabel(shibuyaContractLabel);
    const shibuyaContractAddress: any = localStorage.getItem('shibuyaContractAddress');
    setShibuyaContractAddress(shibuyaContractAddress);
    const localContractLabel: any = localStorage.getItem('localContractLabel');
    setLocalContractLabel(localContractLabel);
    const localContractAddress: any = localStorage.getItem('localContractAddress');
    setLocalContractAddress(localContractAddress);
    const customContractLabel: any = localStorage.getItem('customContractLabel');
    setCustomContractLabel(customContractLabel);
    const customContractAddress: any = localStorage.getItem('customContractAddress');
    setCustomContractAddress(customContractAddress);
  },[]);

  const saveContractInfo = (chain: string, type: string, str: string) => {
    str = str.trim();
    if (chain === 'shiden') {
      if (type === 'label') {
        setShidenContractLabel(str);
        localStorage.setItem('shidenContractLabel', str);
      } else if (type === 'address') {
        setShidenContractAddress(str);
        localStorage.setItem('shidenContractAddress', str);
      }
    } else if (chain === 'shibuya') {
      if (type === 'label') {
        setShibuyaContractLabel(str);
        localStorage.setItem('shibuyaContractLabel', str);
      } else if (type === 'address') {
        setShibuyaContractAddress(str);
        localStorage.setItem('shibuyaContractAddress', str);
      }
    } else if (chain === 'local') {
      if (type === 'label') {
        setLocalContractLabel(str);
        localStorage.setItem('localContractLabel', str);
      } else if (type === 'address') {
        setLocalContractAddress(str);
        localStorage.setItem('localContractAddress', str);
      }
    } else if (chain === 'custom') {
      if (type === 'label') {
        setCustomContractLabel(str);
        localStorage.setItem('customContractLabel', str);
      } else if (type === 'address') {
        setCustomContractAddress(str);
        localStorage.setItem('customContractAddress', str);
      }
    }
  };

  return (
    <div className="text-left max-w-6xl p-2 m-auto mt-5 w-11/12 border-[#d8d2c5] dark:border-[#323943] bg-[#f4efe2] dark:bg-[#121923] border border-1 rounded">
      <h3 className="m-1 text-xl text-center">Sample Contracts</h3>
      <dl role="list" className="m-1 break-all">
        <dt className="m-1 text-xl">Shiden</dt>
        {navigation.shiden.map((item) => (
          <dd className="m-2 ml-6" key={item.name}>{item.name}: <span>{item.address}</span></dd>
        ))}
        <dd className="ml-4">
          <div className='w-[95%] max-w-[700px]'>
            <input
                className="w-[45%] md:w-[30%] md:max-w-[150px] p-2 m-2 mr-0 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
                onChange={(event) => saveContractInfo('shiden', 'label', event.target.value)}
                placeholder="Label"
                value={shidenContractLabel}
            />
            <input
                className="w-[95%] md:w-[70%] md:max-w-[500px] p-2 m-2 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
                onChange={(event) => saveContractInfo('shiden', 'address', event.target.value)}
                placeholder="Shiden contract address here"
                value={shidenContractAddress}
            />
          </div>
        </dd>
      </dl>
      <dl role="list" className="mt-3 m-1 break-all">
        <dt className="m-1 text-xl">Shibuya</dt>
        {navigation.shibuya.map((item) => (
          <dd className="m-2 ml-6" key={item.name}>{item.name} :  <span>{item.address}</span></dd>
        ))}
        <dd className="ml-4">
          <div className='w-[95%] max-w-[700px]'>
            <input
                className="w-[45%] md:w-[30%] md:max-w-[150px] p-2 m-2 mr-0 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
                onChange={(event) => saveContractInfo('shibuya', 'label', event.target.value)}
                placeholder="Label"
                value={shibuyaContractLabel}
            />
            <input
                className="w-[95%] md:w-[70%] md:max-w-[500px] p-2 m-2 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
                onChange={(event) => saveContractInfo('shibuya', 'address', event.target.value)}
                placeholder="Shibuya contract address here"
                value={shibuyaContractAddress}
            />
          </div>
        </dd>
      </dl>
      <dl role="list" className="mt-3 m-1 break-all">
        <dt className="m-1 text-xl">Local</dt>
        <dd className="ml-4">
          <div className='w-[95%] max-w-[700px]'>
            <input
                className="w-[45%] md:w-[30%] md:max-w-[150px] p-2 m-2 mr-0 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
                onChange={(event) => saveContractInfo('local', 'label', event.target.value)}
                placeholder="Label"
                value={localContractLabel}
            />
            <input
                className="w-[95%] md:w-[70%] md:max-w-[500px] p-2 m-2 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
                onChange={(event) => saveContractInfo('local', 'address', event.target.value)}
                placeholder="Local contract address here"
                value={localContractAddress}
            />
          </div>
        </dd>
      </dl>
      <dl role="list" className="mt-3 m-1 break-all">
        <dt className="m-1 text-xl">Custom</dt>
        <dd className="ml-4">
          <div className='w-[95%] max-w-[700px]'>
            <input
                className="w-[45%] md:w-[30%] md:max-w-[150px] p-2 m-2 mr-0 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
                onChange={(event) => saveContractInfo('custom', 'label', event.target.value)}
                placeholder="Label"
                value={customContractLabel}
            />
            <input
                className="w-[95%] md:w-[70%] md:max-w-[550px] p-2 m-2 bg-[#dcd6c8] dark:bg-[#020913] border-2 border-[#95928b] dark:border-gray-500 rounded"
                onChange={(event) => saveContractInfo('custom', 'address', event.target.value)}
                placeholder="Custom contract address here"
                value={customContractAddress}
            />
          </div>
        </dd>
      </dl>
    </div>
  );
};

export default SampleContractsList;