mod utils;
use std::iter::{self, successors, Successors};

use crate::utils::*;

include!(concat!(env!("OUT_DIR"), "/sboxes.rs"));

fn compress(char: [u64; 8], state: &mut [u64; 3]) {
    compress_with_sbox(char, state, SBOXES)
}

fn hash(bytes: Vec<u8>) -> String {
    let mut state = START_VALUES;
    let message_len = (bytes.len() & 0xFFFFFFFF) as u64;

    let mut compress_chunk = |chunk: &[u8]| {
        let char: [u64; 8] = chunk
            .chunks_exact(8)
            .map(|c| u64::from_le_bytes(c.try_into().unwrap()))
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap();
        compress(char, &mut state);
    };

    let mut iter = bytes.chunks_exact(64);
    iter.by_ref().for_each(&mut compress_chunk);
    let mut remainder = iter.remainder().to_vec();
    remainder.push(0x01);
    remainder
        .extend(iter::repeat(0x00).take((56 - remainder.len() as i64).rem_euclid(64) as usize));
    remainder.append(&mut (message_len << 3).to_le_bytes().to_vec());
    remainder.chunks_exact(64).for_each(&mut compress_chunk);

    let state = state.map(|x| u64::from_le_bytes(x.to_be_bytes()));

    format!("{:016X}{:016X}{:016X}", state[0], state[1], state[2])
}

fn xrange(
    start: i64,
    end: i64,
    step: i64,
) -> std::iter::Successors<i64, impl FnMut(&i64) -> Option<i64>> {
    successors(
        if step > 0 && start < end || step < 0 && start > end {
            Some(start)
        } else {
            None
        },
        move |&i| {
            let next = i + step;
            if step > 0 && next < end || step < 0 && next > end {
                Some(next)
            } else {
                None
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let test_string = |str: String, expected_hash: &str| {
            assert_eq!(hash(str.into_bytes()), expected_hash);
        };

        test_string(
            "".to_string(),
            "3293AC630C13F0245F92BBB1766E16167A4E58492DDE73F3",
        );
        test_string(
            "a".to_string(),
            "77BEFBEF2E7EF8AB2EC8F93BF587A7FC613E247F5F247809",
        );
        test_string(
            "abc".to_string(),
            "2AAB1484E8C158F2BFB8C5FF41B57A525129131C957B5F93",
        );
        test_string(
            "message digest".to_string(),
            "D981F8CB78201A950DCF3048751E441C517FCA1AA55A29F6",
        );
        test_string(
            "abcdefghijklmnopqrstuvwxyz".to_string(),
            "1714A472EEE57D30040412BFCC55032A0B11602FF37BEEE9",
        );
        test_string(
            "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".to_string(),
            "0F7BF9A19B9C58F2B7610DF7E84F0AC3A71C631E7B53F78E",
        );
        test_string(
            ("abcdefghijklmnopqrstuvwxyz".to_ascii_uppercase()
                + "abcdefghijklmnopqrstuvwxyz"
                + "0123456789")
                .to_string(),
            "8DCEA680A17583EE502BA38A3C368651890FFBCCDC49A8CC",
        );
        test_string(
            "1234567890".repeat(8).to_string(),
            "1C14795529FD9F207A958F84C52F11E887FA0CABDFD91BFD",
        );
        test_string("a".repeat(1_000_000).to_string(), "6DB0E2729CBEAD93D715C6A7D36302E9B3CEE0D2BC314B41");

        assert_eq!(
            hash([0; 0].to_vec()),
            "3293AC630C13F0245F92BBB1766E16167A4E58492DDE73F3"
        );
        assert_eq!(
            hash([0; 1].to_vec()),
            "5D9ED00A030E638BDB753A6A24FB900E5A63B8E73E6C25B6"
        );
        assert_eq!(
            hash([0; 2].to_vec()),
            "AABBCCA084ACECD0511D1F6232A17BFAEFA441B2982E5548"
        );
        assert_eq!(
            hash([0; 3].to_vec()),
            "7F7A8CE9580077EB6FD53BE8BB3E70650E2FDD8DBB44F5C7"
        );
        assert_eq!(
            hash([0; 4].to_vec()),
            "605D1B8C132BF5D16F1A8BC2451733F7F0FF57FD5F49E298"
        );
        assert_eq!(
            hash([0; 5].to_vec()),
            "7F3A0954A5C374566C72370D1D97C2AC8FA9B1E3CB216E31"
        );
        assert_eq!(
            hash([0; 6].to_vec()),
            "B333735AFDAB30B4B597CCD137AC1AD2B30D4A047BAD0127"
        );
        assert_eq!(
            hash([0; 7].to_vec()),
            "1F27BB5926F35CF5D52A24817BF56CD17D79BB65ABE85785"
        );
        assert_eq!(
            hash([0; 8].to_vec()),
            "5229DC51A494B913F8E04C4C729C93CB8B260CA4EE8EA9D7"
        );
        assert_eq!(
            hash([0; 9].to_vec()),
            "5DFE02959A64534E0E679937B712F7DDE2C7B982597D315D"
        );
        assert_eq!(
            hash([0; 10].to_vec()),
            "654D00C774FBE74CFE4C0A6DEA5917823A6B20070CD0CCCB"
        );
        assert_eq!(
            hash([0; 11].to_vec()),
            "C82A6419844FC7028B52CAC8186D63B85121AAACBA622A87"
        );
        assert_eq!(
            hash([0; 12].to_vec()),
            "06F6D65EF081AFE892AC0946F275DBAB9D3B6D9C55E517C9"
        );
        assert_eq!(
            hash([0; 13].to_vec()),
            "42C40537A876D69DF84FF36FBA3127901B112B9E96688012"
        );
        assert_eq!(
            hash([0; 14].to_vec()),
            "6FA00EB82505EBC4707F675B0A72A9F9795F647B550F155D"
        );
        assert_eq!(
            hash([0; 15].to_vec()),
            "239549439CCE539F304CB7A95160A033529B228CA406EFF9"
        );
        assert_eq!(
            hash([0; 16].to_vec()),
            "464B87921CCDAEDBC0D6941610D1EB19E536036096403F32"
        );
        assert_eq!(
            hash([0; 17].to_vec()),
            "F29CF9B53DA4F9955E7BB766EDBAD59EA914E5354DDF9ACF"
        );
        assert_eq!(
            hash([0; 18].to_vec()),
            "E9B6BAA124BA93A4501DB9A92F212A14C538CF82C17E89B4"
        );
        assert_eq!(
            hash([0; 19].to_vec()),
            "03EB6D4703EFCF2E088515D3B902A1CF78711B7E7CE3AF74"
        );
        assert_eq!(
            hash([0; 20].to_vec()),
            "53BFCB1EB5CA9B2E8A4225E6C00584023FF8AA0017AD3A0D"
        );
        assert_eq!(
            hash([0; 21].to_vec()),
            "67F6D2B74B13251432469B9CE6A7C91EAD6E72B9CB51CEF8"
        );
        assert_eq!(
            hash([0; 22].to_vec()),
            "81A68B237DDC60C9C31B34B52A754A8A7B1F51D40781B988"
        );
        assert_eq!(
            hash([0; 23].to_vec()),
            "E99E18449AEB4421CEC31D1521C23445BC2598EA5DA14A09"
        );
        assert_eq!(
            hash([0; 24].to_vec()),
            "CDDDCACFEA7B70B485655BA3DC3F60DEE4F6B8F861069E33"
        );
        assert_eq!(
            hash([0; 25].to_vec()),
            "424CDDE2E5478A29A71352AAE538EAFE9F1B6F07AE05137B"
        );
        assert_eq!(
            hash([0; 26].to_vec()),
            "996B4DD292782A19D742B534916870023F32E72F1C197625"
        );
        assert_eq!(
            hash([0; 27].to_vec()),
            "2C4B719E5E0BFBBA616FB8AB35470D84B676C7D3A6395262"
        );
        assert_eq!(
            hash([0; 28].to_vec()),
            "F178E06FC3319BA18F2EAC3DCC5240D06BAC062BCD5CA1CE"
        );
        assert_eq!(
            hash([0; 29].to_vec()),
            "E899374C34AED37FBC9E0E4A0D37A13045639D5CAF24095A"
        );
        assert_eq!(
            hash([0; 30].to_vec()),
            "3D8E74F26682750FEEDE30A305C9CA162C6BDD9B790BCC5D"
        );
        assert_eq!(
            hash([0; 31].to_vec()),
            "3EFCD915BB46020B8999C9AD56356FB91F93C4907C4012D5"
        );
        assert_eq!(
            hash([0; 32].to_vec()),
            "739414BD4CD6AB967CD46A1D943412757D858B24D1C4ECF7"
        );
        assert_eq!(
            hash([0; 33].to_vec()),
            "3615FE2BB3B94A00245A1B3DF68B9D43BBAD82B16BFCD6A5"
        );
        assert_eq!(
            hash([0; 34].to_vec()),
            "9E9AD6783D3775885E65D05EDA4262E179836ACBB9867CE2"
        );
        assert_eq!(
            hash([0; 35].to_vec()),
            "5D91C8314556534FF3DEE0515334F09E16935538E347C029"
        );
        assert_eq!(
            hash([0; 36].to_vec()),
            "E48325D26CBEA5C14AF5D2BC043242DE2A5DC17EF5798BA0"
        );
        assert_eq!(
            hash([0; 37].to_vec()),
            "0DE25B928CF3E4FE85F46C18334D9C9DAE995E4889068C4E"
        );
        assert_eq!(
            hash([0; 38].to_vec()),
            "1BD4360038C1282E66538B156E9F604AA0262D4608184A5C"
        );
        assert_eq!(
            hash([0; 39].to_vec()),
            "FCC72686AD012FF48D1471CD0EAD606EC21A5E496DB66103"
        );
        assert_eq!(
            hash([0; 40].to_vec()),
            "F11D39950658DBDF786703E3EDAAA3A654B4428AD419DF11"
        );
        assert_eq!(
            hash([0; 41].to_vec()),
            "2AFACE18DA68984D907D1C0F647E9244D6976F4F471064B9"
        );
        assert_eq!(
            hash([0; 42].to_vec()),
            "E2056969720C08B5C9C8F903416668A7F5B2D6F95046C5FF"
        );
        assert_eq!(
            hash([0; 43].to_vec()),
            "D95BC879A68D549A5296BD34344C5ACC4D826E54ECEC346C"
        );
        assert_eq!(
            hash([0; 44].to_vec()),
            "D33A7246729743643BF4B438FBB9BC2374F9E346A0868DC6"
        );
        assert_eq!(
            hash([0; 45].to_vec()),
            "1201A8247225F31B928390B35787C046653249D0441F0AAD"
        );
        assert_eq!(
            hash([0; 46].to_vec()),
            "1E63B6124EF57A92B36C8FF837C38CC9072048ED98EE853F"
        );
        assert_eq!(
            hash([0; 47].to_vec()),
            "F2E4B6637FDFF18F785894EECEA514E7B96B2409E62DF3FB"
        );
        assert_eq!(
            hash([0; 48].to_vec()),
            "10DD94B66BA6AE0498C9C7754844662E5D8B62E27D2C4D26"
        );
        assert_eq!(
            hash([0; 49].to_vec()),
            "8BD37E3CF05E59537389672DF921392A45CB57ACFE9247A1"
        );
        assert_eq!(
            hash([0; 50].to_vec()),
            "41621DF28B038C0032508592F986C7C4832927CD1134B2B4"
        );
        assert_eq!(
            hash([0; 51].to_vec()),
            "978365EDBC762C10187F212BF9115D79FCBD489EC135AC9A"
        );
        assert_eq!(
            hash([0; 52].to_vec()),
            "8C4A9C341016B2D74E1DC75478A20A57EC934C38E6D60DF3"
        );
        assert_eq!(
            hash([0; 53].to_vec()),
            "FD5A1DDED7D38B987B582FD91E95FF8ED84DDF5D486A00EB"
        );
        assert_eq!(
            hash([0; 54].to_vec()),
            "F5845CBBA386319361D4042658606947FB96D72934378AA3"
        );
        assert_eq!(
            hash([0; 55].to_vec()),
            "A4EE394B2A208E9B0A74C6D57568E470F6E658C44689FC63"
        );
        assert_eq!(
            hash([0; 56].to_vec()),
            "19208AEF976EEA1A1296AB46BB8519E4E35CC3D26D2B574F"
        );
        assert_eq!(
            hash([0; 57].to_vec()),
            "A0D74C6090806AE1B4A9A676950B940DC37D28F762AF66DC"
        );
        assert_eq!(
            hash([0; 58].to_vec()),
            "DA1DCA0F6E414B6634ECF34DE132F3320BE35AC8D6AB0BBB"
        );
        assert_eq!(
            hash([0; 59].to_vec()),
            "49D948E0A5149A7B41F73C30B62FEEDBABA7DD9FFA9DB383"
        );
        assert_eq!(
            hash([0; 60].to_vec()),
            "95475FE7F5016A6FBB175EBF3B13EFE41FCFE9249586BB2A"
        );
        assert_eq!(
            hash([0; 61].to_vec()),
            "8AEEF6FEB5BF02F8ACC6430FEC5CC7858EEDF9C1DBBE8F09"
        );
        assert_eq!(
            hash([0; 62].to_vec()),
            "0E425380B928052574D82A3F604162F3021361517560271F"
        );
        assert_eq!(
            hash([0; 63].to_vec()),
            "A857DD168B22B65A6DD2EA8035C4EDC4B890453D14BA6052"
        );
        assert_eq!(
            hash([0; 64].to_vec()),
            "33FF9966DDD692427A9BC4D611F3C74CF629A0544A1A7ED7"
        );
        assert_eq!(
            hash([0; 65].to_vec()),
            "759D79778FA748F8E828A568C45B7E2774A6052E0A4A06A7"
        );
        assert_eq!(
            hash([0; 66].to_vec()),
            "93E4986EC1BAE0334662495E3261769214DFAB96C6567A12"
        );
        assert_eq!(
            hash([0; 67].to_vec()),
            "D50D44D48A7519B21C7ACF59DCD56066C7C328F680F2FB7C"
        );
        assert_eq!(
            hash([0; 68].to_vec()),
            "6C240CDCC41A5DB56F7A88E25A04A2F3D10D33B6908D0F4C"
        );
        assert_eq!(
            hash([0; 69].to_vec()),
            "D8581CF122C23E06A9A8781E6635113F76CBF21ECC520BE7"
        );
        assert_eq!(
            hash([0; 70].to_vec()),
            "1E763325671BEE3E17FD894D7A4E1405315004563BFAA368"
        );
        assert_eq!(
            hash([0; 71].to_vec()),
            "57B581CBF0EF91E38F4A306FFB3791A0BE6E1FA96BDE0703"
        );
        assert_eq!(
            hash([0; 72].to_vec()),
            "0870537F7FD0A5F600E2B02B31878C2244F2E76C8D0AB421"
        );
        assert_eq!(
            hash([0; 73].to_vec()),
            "6F1D28AA4EEF347CAE41FABBC528AE71345BAC83D6572BAF"
        );
        assert_eq!(
            hash([0; 74].to_vec()),
            "704CE7D9895218DAABAF9BDF944F7901571D0C24873984BF"
        );
        assert_eq!(
            hash([0; 75].to_vec()),
            "1574D6AD7CABC7DFAEE1520289E0ED8B26DC6195C5F588BD"
        );
        assert_eq!(
            hash([0; 76].to_vec()),
            "257196F102E891E04ACD046CF099A2191ACE7F878B1C5CE2"
        );
        assert_eq!(
            hash([0; 77].to_vec()),
            "359F480A3475F89093F33289C3EDD28867C0E0F11AF79939"
        );
        assert_eq!(
            hash([0; 78].to_vec()),
            "8CA6EF62DD4E1445C729E3FAFF0B57DF5ADA90D714B906F6"
        );
        assert_eq!(
            hash([0; 79].to_vec()),
            "D917B8C5CF18FBD88482F6754EFC9D308EBCAB912A609D5E"
        );
        assert_eq!(
            hash([0; 80].to_vec()),
            "B3950A9D299A5A732CC0841F1EFAE62F3DB20A707B98F3F5"
        );
        assert_eq!(
            hash([0; 81].to_vec()),
            "4112B496A4BA7C67F040A48A30C3F48496FDD3AD2A4A9E6B"
        );
        assert_eq!(
            hash([0; 82].to_vec()),
            "D663C288232D8512986F0C2F37F20A764AFABF068F44CE62"
        );
        assert_eq!(
            hash([0; 83].to_vec()),
            "4B0292082184E5727B98ABCFD57197EE14AB5893E2CDD370"
        );
        assert_eq!(
            hash([0; 84].to_vec()),
            "9EAF05D45A302F909E6850E28AF8A88E51799E20EF75D0E3"
        );
        assert_eq!(
            hash([0; 85].to_vec()),
            "AB8FBE9C09E9906200201F3AFCEF79AFF29267BA19C63A82"
        );
        assert_eq!(
            hash([0; 86].to_vec()),
            "DB76DF9A0D2E9D21B57EC44C99653F59348981467EC2476D"
        );
        assert_eq!(
            hash([0; 87].to_vec()),
            "0C2CCBAEB70C5C1F85FAFA01B1719F363D902716810F582E"
        );
        assert_eq!(
            hash([0; 88].to_vec()),
            "FCED365E31C7E4F3B06CB6720875A33E1AC45D047D071342"
        );
        assert_eq!(
            hash([0; 89].to_vec()),
            "2FEDFD926783A5C91D51BCC3228F2846243B95E5D858AF07"
        );
        assert_eq!(
            hash([0; 90].to_vec()),
            "C1789746C0E04E42F2CEAA374FBCEDCFCB17AFB816D3D9B5"
        );
        assert_eq!(
            hash([0; 91].to_vec()),
            "C055A3A9EF84BD96AEBAE31C08B74BA22BD2489C70D25672"
        );
        assert_eq!(
            hash([0; 92].to_vec()),
            "AE1AEECA1373D4348E4EDC2B7F393A8F44A351AA952AD7F7"
        );
        assert_eq!(
            hash([0; 93].to_vec()),
            "60CA04458514989969B5D3A1537F226CB54E3FCE9CC9B134"
        );
        assert_eq!(
            hash([0; 94].to_vec()),
            "20E29A4A4C36D7C2BADE368564765EE0353709CE47D0E36A"
        );
        assert_eq!(
            hash([0; 95].to_vec()),
            "8CE7840450B26DE74B798056D30584DCC436AD79B061729D"
        );
        assert_eq!(
            hash([0; 96].to_vec()),
            "05D2A28308C44536C591DF21B02AB542AF982B81A4C5E129"
        );
        assert_eq!(
            hash([0; 97].to_vec()),
            "196ACD3484E2F5FD51AC0CC667596AB0E764936C37A345B7"
        );
        assert_eq!(
            hash([0; 98].to_vec()),
            "C4F29A86A5415A0086FEC47CB3D46A71E8D224880657789A"
        );
        assert_eq!(
            hash([0; 99].to_vec()),
            "C5AD956C7D06EC8651665176733FB2EF0D01BE7E81E0F775"
        );
        assert_eq!(
            hash([0; 100].to_vec()),
            "A5A54E12A9538A158E78AE09896DCB2CE31F14150625E615"
        );
        assert_eq!(
            hash([0; 101].to_vec()),
            "239045220EF064D13637D503A6079EF21B5F2D02CF98FA5A"
        );
        assert_eq!(
            hash([0; 102].to_vec()),
            "AAEA04D2B11931D58016A9B68B3FF543C7BFD87823EA6ABA"
        );
        assert_eq!(
            hash([0; 103].to_vec()),
            "37838610BB3944FB3DFBB14B95A4007705B3773148E58AC3"
        );
        assert_eq!(
            hash([0; 104].to_vec()),
            "3DE1ECB5FEC022C7CF8B4CF4AFFEE41D85648EAEF6B0F5A7"
        );
        assert_eq!(
            hash([0; 105].to_vec()),
            "40224B9C7ECBBB69D1A5E0A3A0F249315AE1E7DE859FD763"
        );
        assert_eq!(
            hash([0; 106].to_vec()),
            "7EF8F229B824EA82F8E8834E799A67A0B946086BD9AA582B"
        );
        assert_eq!(
            hash([0; 107].to_vec()),
            "2543ECA4058C02EFE62E2131E9A7627D6DCAA6C51A13A517"
        );
        assert_eq!(
            hash([0; 108].to_vec()),
            "51E2B3A3D56D88BDB4F77CEAC0D146B08C269EB50D7914D4"
        );
        assert_eq!(
            hash([0; 109].to_vec()),
            "9A3A264F48D2BC4B08671B0D6EE58B4E0E87D19CFC561738"
        );
        assert_eq!(
            hash([0; 110].to_vec()),
            "F8F8F7095F7FF1111811FB2B63F431B529CA740EFA9A3350"
        );
        assert_eq!(
            hash([0; 111].to_vec()),
            "E0BF4F81FB633EBF0092E6AD4EC8B6834A85DF4F639F94D0"
        );
        assert_eq!(
            hash([0; 112].to_vec()),
            "114BD7F8FCA026BA9874DD79174EA1FB7275B8C7F2042871"
        );
        assert_eq!(
            hash([0; 113].to_vec()),
            "BDE5124EBA58CD1929DD63E9A4A987296D93E78378BA7E21"
        );
        assert_eq!(
            hash([0; 114].to_vec()),
            "FAA1FA47DEDC888043BC35BD97288C3E3E8702E3790A5ED4"
        );
        assert_eq!(
            hash([0; 115].to_vec()),
            "4D0016CD63CCB3C917A9CAC0A656ABF1BF1F79A42D9FB4E5"
        );
        assert_eq!(
            hash([0; 116].to_vec()),
            "1D9C9AA4BD15F3B6DDDE9771259AA550F17CE01383DFC3E7"
        );
        assert_eq!(
            hash([0; 117].to_vec()),
            "D70BCAA1810D6213A77935D751F52C8743C809C37DCB02CE"
        );
        assert_eq!(
            hash([0; 118].to_vec()),
            "9759AE86B5CC71DB9406A2ABDD0537940D717E2953E4573F"
        );
        assert_eq!(
            hash([0; 119].to_vec()),
            "A6452EF736D5CFF79BC3C3DBAA7FC7B4B992AE7DBD87650B"
        );
        assert_eq!(
            hash([0; 120].to_vec()),
            "FB7177FFE0E718C91348A8302C57129596149F042FCC99A6"
        );
        assert_eq!(
            hash([0; 121].to_vec()),
            "A6FABD413D9FAF90AC0A8D97F82B1D28761D9B3A7A3A2E2C"
        );
        assert_eq!(
            hash([0; 122].to_vec()),
            "93E56AE559EC7BF41251B6C84700465BF78E0757576679B3"
        );
        assert_eq!(
            hash([0; 123].to_vec()),
            "24F342F67741BFD5CCA5E9C5C0D902CB6F93DF12203E891A"
        );
        assert_eq!(
            hash([0; 124].to_vec()),
            "BE7803AE8A8C84B7567352C2E3A78BE424784406D33F6930"
        );
        assert_eq!(
            hash([0; 125].to_vec()),
            "43A59ECAB48D3784CD83B0887E92D208A0E7CEC6DA3A7146"
        );
        assert_eq!(
            hash([0; 126].to_vec()),
            "7D16DC5E00379CA0F13FA261AD5FDB56449ADF9DAD3810E1"
        );
        assert_eq!(
            hash([0; 127].to_vec()),
            "00A569CCADB17662483CA36230BCC6956BA5C1D5595C044A"
        );
    }

    #[test]
    fn test_chunks() {
        let block = [0x0123456789ABCDEF; 8];
        assert_eq!(block, readChunks(writeChunks(block)))
    }

    #[test]
    fn test_xrange() {
        let start: i64 = 0;
        let end: i64 = 10;
        let step: i64 = 2;
        let expected_output: Vec<i64> = vec![0, 2, 4, 6, 8];
        assert_eq!(
            xrange(start, end, step).collect::<Vec<i64>>(),
            expected_output
        );

        let start: i64 = 10;
        let end: i64 = 0;
        let step: i64 = -2;
        let expected_output: Vec<i64> = vec![10, 8, 6, 4, 2];
        assert_eq!(
            xrange(start, end, step).collect::<Vec<i64>>(),
            expected_output
        );

        let start: i64 = 10;
        let end: i64 = -1;
        let step: i64 = -2;
        let expected_output: Vec<i64> = vec![10, 8, 6, 4, 2, 0];
        assert_eq!(
            xrange(start, end, step).collect::<Vec<i64>>(),
            expected_output
        );

        let start: i64 = 10;
        let end: i64 = -5;
        let step: i64 = -2;
        let expected_output: Vec<i64> = vec![10, 8, 6, 4, 2, 0, -2, -4];
        assert_eq!(
            xrange(start, end, step).collect::<Vec<i64>>(),
            expected_output
        );
    }
}
