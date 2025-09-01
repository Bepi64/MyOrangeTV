use serde::Deserialize;
use serde_xml_rs::from_str;

#[derive(Debug, Deserialize)]
pub struct Root {
    pub device: Device,
}

#[derive(Debug, Deserialize)]
pub struct Device {
    pub friendlyName: String,
    pub serviceList: ServiceList,
}

#[derive(Debug, Deserialize)]
pub struct ServiceList {
    pub service: Service,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub controlURL: String,
    pub SCPDURL: String,
}

/*fn main() {
    let xml = r#"
<root>
    <specVersion>
        <major>1</major>
        <minor>0</minor>
    </specVersion>
    <device>
        <deviceType>urn:schemas-upnp-org:device:Basic:1</deviceType>
        <friendlyName>décodeur TV 4</friendlyName>
        <manufacturer>SAGEM</manufacturer>
        <modelName>WHD93</modelName>
        <UDN>uuid:78bd2958-8f09-51bf-a09d-3ffcf11cdcc2</UDN>
        <serviceList>
            <service>
                <serviceType></serviceType>
                <serviceId>urn:orange-com:serviceId:X_OrangeSTBRemoteControl</serviceId>
                <SCPDURL>/X_OrangeSTBRemoteControlDescription.xml</SCPDURL>
                <controlURL>/remoteControl/cmd</controlURL>
                <eventSubURL>/remoteControl/notifyEvent</eventSubURL>
            </service>
        </serviceList>
    </device>
</root>
"#;

    let parsed: Root = from_str(xml).unwrap();
    println!("Nom convivial : {}", parsed.device.friendlyName);
    println!("URL de contrôle : {}", parsed.device.serviceList.service.controlURL);
}*/
