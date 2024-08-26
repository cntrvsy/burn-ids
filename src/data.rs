use burn::{
    data::{
        dataloader::batcher::Batcher,
        dataset::{
            transform::{PartialDataset, ShuffledDataset},
            Dataset, HuggingfaceDatasetLoader, SqliteDataset,
        },
    },
    prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnswNb15Item {
    // NB: the following is what we take from the dataset in USW-NB15_features.csv (https://unsw-my.sharepoint.com/personal/z5025758_ad_unsw_edu_au/_layouts/15/onedrive.aspx?ga=1&id=%2Fpersonal%2Fz5025758%5Fad%5Funsw%5Fedu%5Fau%2FDocuments%2FUNSW%2DNB15%20dataset%2FCSV%20Files)
    //no srcip = source IP address,
    //no sport = source port,
    //no dsport = destination IP address,
    //no stcpb = Source TCP base sequence number,
    //no dtcpb = Destination TCP base sequence number,
    //no res_bdy_len = Actual uncompressed content size of the data transferred from the server’s http service,
    //no Stime = record start time,
    //no Ltime = record last time,
    //no Sintpkt = Source interpacket arrival time (mSec),
    //no Dintpkt = Destination interpacket arrival time (mSec),
    //no is_sm_ips_ports = If source (1) and destination (3)IP addresses equal and port numbers (2)(4)  equal then, this variable takes value 1 else 0
    //no ct_dst_ltm = No. of connections of the same destination address (3) in 100 connections according to the last time (26).
    //no ct_src_dport_ltm = No of connections of the same source address (1) and the destination port (4) in 100 connections according to the last time (26).
    //no ct_dst_sport_ltm = No of connections of the same destination address (3) and the source port (2) in 100 connections according to the last time (26).
    //no ct_dst_src_ltm = No of connections of the same source (1) and the destination (3) address in in 100 connections according to the last time (26).

    //dur = Float = Record total duration
    #[serde(rename = "dur")]
    pub dur: f64,

    //proto = String = Network/Transaction protocol
    #[serde(rename = "proto")]
    pub proto: String,

    //service = String = Network service (http, ftp, smtp, ssh, dns, ftp-data ,irc  and (-) if not much used service)
    #[serde(rename = "service")]
    pub service: String,

    //state = String = TCP connection state (Indicates to the state and its dependent protocol, e.g. ACC, CLO, CON, ECO, ECR, FIN, INT, MAS, PAR, REQ, RST, TST, TXD, URH, URN, and (-) (if not used state))
    #[serde(rename = "state")]
    pub state: String,

    //spkts = Int = Number of packets sent (Source to destination packet count )
    #[serde(rename = "spkts")]
    pub spkts: f64,

    //dpkts = Int = Number of packets received (Destination to source packet count)
    #[serde(rename = "dpkts")]
    pub dpkts: f64,

    //sbytes = Int = Number of bytes sent (Source to destination transaction bytes )
    #[serde(rename = "sbytes")]
    pub sbytes: f64,

    //dbytes = Int = Number of bytes received (Destination to source transaction bytes)
    #[serde(rename = "dbytes")]
    pub dbytes: f64,

    //rate = Int = Rate
    #[serde(rename = "rate")]
    pub rate: f64,

    //sttl = Int = Time to live (Source to destination time to live value )
    #[serde(rename = "sttl")]
    pub sttl: f64,

    //dttl = Int = Time to live (Destination to source time to live value)
    #[serde(rename = "dttl")]
    pub dttl: f64,

    //sload = Float = Data rate (Source to destination data rate )
    #[serde(rename = "sload")]
    pub sload: f64,

    //dload = Float = Data rate (Destination to source data rate)
    #[serde(rename = "dload")]
    pub dload: f64,

    //sloss = Int = Packets lost in send (Source packets retransmitted or dropped)
    #[serde(rename = "sloss")]
    pub sloss: f64,

    //dloss = Int = Packets lost in receive (Destination packets retransmitted or dropped)
    #[serde(rename = "dloss")]
    pub dloss: f64,

    //sinpkt = Int = Number of packets sent in
    #[serde(rename = "sinpkt")]
    pub sinpkt: f64,

    //dinpkt = Int = Number of packets received in
    #[serde(rename = "dinpkt")]
    pub dinpkt: f64,

    //sjit = Int = Jitter in send (Source jitter (mSec))
    #[serde(rename = "sjit")]
    pub sjit: f64,

    //djit = Int = Jitter in receive (Destination jitter (mSec))
    #[serde(rename = "djit")]
    pub djit: f64,

    //swin = Int = Window in send (Source TCP window advertisement value)
    #[serde(rename = "swin")]
    pub swin: f64,

    //dwin = Int = Window in receive (Destination TCP window advertisement value)
    #[serde(rename = "dwin")]
    pub dwin: f64,

    //tcprtt = Int = TCP RTT (TCP connection setup round-trip time, the sum of ’synack’ and ’ackdat’.)
    #[serde(rename = "tcprtt")]
    pub tcprtt: f64,

    //stcpb = Int = TCP packets sent
    #[serde(rename = "stcpb")]
    pub stcpb: f64,

    //dtcpb = Int = TCP packets received
    #[serde(rename = "dtcpb")]
    pub dtcpb: f64,

    //synack = Int = SYNACK (TCP connection setup time, the time between the SYN and the SYN_ACK packets.)
    #[serde(rename = "synack")]
    pub synack: f64,

    //ackdat = Int = ACKDAT (TCP connection setup time, the time between the SYN_ACK and the ACK packets.)
    #[serde(rename = "ackdat")]
    pub ackdat: f64,

    //smean = Int = Mean send (Retrieving data. Wait a few seconds and try to cut or copy again.)
    #[serde(rename = "smean")]
    pub smean: f64,

    //dmean = Int = Mean receive (Mean of the ?ow packet size transmitted by the dst )
    #[serde(rename = "dmean")]
    pub dmean: f64,

    //trans_depth = Int = Transmission depth (Represents the pipelined depth into the connection of http request/response transaction)
    #[serde(rename = "trans_depth")]
    pub trans_depth: f64,

    //response_body_len = Int = Response body length
    #[serde(rename = "response_body_len")]
    pub response_body_len: f64,

    //ct_srv_src = Int = Source service (No. of connections that contain the same service (14) and source address (1) in 100 connections according to the last time (26).)
    #[serde(rename = "ct_srv_src")]
    pub ct_srv_src: f64,

    //ct_state_ttl = Int = TTL (No. for each state (6) according to specific range of values for source/destination time to live (10) (11).)
    #[serde(rename = "ct_state_ttl")]
    pub ct_state_ttl: f64,

    //ct_flw_http_mthd = Int = HTTP method (No. of flows that has methods such as Get and Post in http service.)
    #[serde(rename = "ct_flw_http_mthd")]
    pub ct_flw_http_mthd: f64,

    //ct_src_ltm = Int = Local time (No. of connections of the same source address (1) in 100 connections according to the last time (26).)
    #[serde(rename = "ct_src_ltm")]
    pub ct_src_ltm: f64,

    //ct_srv_dst = Int = Destination service (No. of connections that contain the same service (14) and destination address (3) in 100 connections according to the last time (26).)
    #[serde(rename = "ct_srv_dst")]
    pub ct_srv_dst: f64,

    //is_ftp_login = Int = FTP login (If the ftp session is accessed by user and password then 1 else 0.)
    #[serde(rename = "is_ftp_login")]
    pub is_ftp_login: f64,

    //ct_ftp_cmd = Int = FTP command (No of flows that has a command in ftp session.)
    #[serde(rename = "ct_ftp_cmd")]
    pub ct_ftp_cmd: f64,

    //ct_dst_ltm = Int = Local time (No. of connections of the same destination address (3) in 100 connections according to the last time (26).)
    #[serde(rename = "ct_dst_ltm")]
    pub ct_dst_ltm: f64,

    //ct_src_dport_ltm = Int = Local port (No. of connections of the same source port (2) in 100 connections according to the last time (26).)
    #[serde(rename = "ct_src_dport_ltm")]
    pub ct_src_dport_ltm: f64,

    //ct_dst_src_ltm = Int = Local port (No. of connections of the same source port (2) in 100 connections according to the last time (26).)
    #[serde(rename = "ct_dst_src_ltm")]
    pub ct_dst_src_ltm: f64,

    //label = String = Label (0 for normal and 1 for attack records)
    #[serde(rename = "label")]
    pub label: f64,

    //attack_cat = String = Attack category (The name of each attack category. In this data set , nine categories e.g. Fuzzers, Analysis, Backdoors, DoS Exploits, Generic, Reconnaissance, Shellcode and Worms)
    #[serde(rename = "attack_cat")]
    pub attack_cat: String,
}

type ShuffledData = ShuffledDataset<SqliteDataset<UnswNb15Item>, UnswNb15Item>;
type PartialData = PartialDataset<ShuffledData, UnswNb15Item>;

pub struct UnswNb15Dataset {
    dataset: PartialData,
}

impl Dataset<UnswNb15Item> for UnswNb15Dataset {
    fn get(&self, index: usize) -> Option<UnswNb15Item> {
        self.dataset.get(index)
    }
    fn len(&self) -> usize {
        self.dataset.len()
    }
}

impl UnswNb15Dataset {
    pub fn train() -> Self {
        Self::new("train")
    }
    pub fn test() -> Self {
        Self::new("test")
    }

    pub fn new(split: &str) -> Self {
        let dataset: SqliteDataset<UnswNb15Item> =
            HuggingfaceDatasetLoader::new("Mireu-Lab/UNSW-NB15")
                .dataset("train")
                .unwrap();

        let len = dataset.len();

        // Shuffle the dataset with a defined seed such that train and test sets have no overlap
        // when splitting by indexes
        let dataset = ShuffledDataset::with_seed(dataset, 42);

        // The dataset from HuggingFace has only train split, so we manually split the train dataset into train
        // and test in a 80-20 ratio

        let filtered_dataset = match split {
            "train" => PartialDataset::new(dataset, 0, len * 8 / 10), // Get first 80% dataset
            "test" => PartialData::new(dataset, len * 8 / 10, len),   // Take remaining 20%
            _ => panic!("Invalid split type"),
        };

        Self {
            dataset: filtered_dataset,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnswNb15Batcher<B: Backend> {
    device: B::Device,
}

#[derive(Clone, Debug)]
pub struct UnswNb15Batch<B: Backend> {
    pub inputs: Tensor<B, 2>,
    pub targets: Tensor<B, 1>,
}

impl<B: Backend> UnswNb15Batcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }

    pub fn min_max_norm<const D: usize>(&self, inp: Tensor<B, D>) -> Tensor<B, D> {
        let min = inp.clone().min_dim(0);
        let max = inp.clone().max_dim(0);
        (inp.clone() - min.clone()).div(max - min)
    }
}

impl<B: Backend> Batcher<UnswNb15Item, UnswNb15Batch<B>> for UnswNb15Batcher<B> {
    fn batch(&self, items: Vec<UnswNb15Item>) -> UnswNb15Batch<B> {
        let mut inputs: Vec<Tensor<B, 2>> = Vec::new();

        for item in items.iter() {
            let input_tensor = Tensor::<B, 1>::from_floats(
                [
                    item.dur as f32,
                    //item.proto,
                    //item.service,
                    //item.state,
                    item.spkts as f32,
                    item.dpkts as f32,
                    item.sbytes as f32,
                    item.dbytes as f32,
                    item.rate as f32,
                    item.sttl as f32,
                    item.dttl as f32,
                    item.sload as f32,
                    item.dload as f32,
                    item.sloss as f32,
                    item.dloss as f32,
                    item.sinpkt as f32,
                    item.dinpkt as f32,
                    item.sjit as f32,
                    item.djit as f32,
                    item.swin as f32,
                    item.stcpb as f32,
                    item.dtcpb as f32,
                    item.tcprtt as f32,
                    item.synack as f32,
                    item.ackdat as f32,
                    item.swin as f32,
                    item.smean as f32,
                    item.dmean as f32,
                    item.trans_depth as f32,
                    item.response_body_len as f32,
                    item.ct_srv_src as f32,
                    item.ct_state_ttl as f32,
                    item.ct_dst_ltm as f32,
                    item.ct_src_dport_ltm as f32,
                    item.ct_dst_src_ltm as f32,
                    item.is_ftp_login as f32,
                    item.ct_ftp_cmd as f32,
                    item.ct_flw_http_mthd as f32,
                    item.ct_src_ltm as f32,
                    item.ct_srv_dst as f32,
                    item.label as f32,
                ],
                &self.device,
            );

            inputs.push(input_tensor.unsqueeze());
        }

        let inputs = Tensor::cat(inputs, 0);
        let inputs = self.min_max_norm(inputs);

        let targets = items
            .iter()
            .map(|item| Tensor::<B, 1>::from_floats([item.label as f32], &self.device))
            .collect();

        let targets = Tensor::cat(targets, 0);
        let targets = self.min_max_norm(targets);

        UnswNb15Batch { inputs, targets }
    }
}
