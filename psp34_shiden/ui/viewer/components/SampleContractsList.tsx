const SampleContractsList = (): JSX.Element => {
  
  const navigation = {
    shiden: [
      { name: 'PSP34Sample', address: 'none' },
    ],
    shibuya: [
      { name: 'PSP34Sample', address: 'aiEWySrv4ufr8NZUvFSWd2Gt7Nn3S6LS1JYieuP1t1ckLz2' },
    ],
    local: [
      { name: 'PSP34Sample', address: 'Input your contract address' },
    ],
  };

  return (
    <div className="text-left max-w-6xl p-2 m-auto mt-5 w-11/12 border-[#323943] bg-[#121923] border border-1 rounded">
      <h3 className="m-1 text-xl text-center">Sample Contracts</h3>
      <dl role="list" className="m-1 break-all">
        <dt className="m-1 text-xl">Shiden</dt>
        {navigation.shiden.map((item) => (
          <dd className="ml-4" key={item.name}>{item.name}: {item.address}</dd>
        ))}
      </dl>
      <dl role="list" className="mt-3 m-1 break-all">
        <dt className="m-1 text-xl">Shibuya</dt>
        {navigation.shibuya.map((item) => (
          <dd className="ml-4" key={item.name}>{item.name}: {item.address}</dd>
        ))}
      </dl>
      <dl role="list" className="mt-3 m-1 break-all">
        <dt className="m-1 text-xl">Local</dt>
        {navigation.local.map((item) => (
          <dd className="ml-4" key={item.name}>{item.name}: {item.address}</dd>
        ))}
      </dl>
    </div>
  );
};

export default SampleContractsList;