import "./styles.css"

export default function Device(props) {
    console.log("Props in device: ", props);
    const { name, mac_addr } = props.device;

    return (
        <>
            <div className="main-device-container ani">
                <p>Name: {name}</p>
                <p>MacAddr: {mac_addr}</p>
            </div>
        </>
    )

}
