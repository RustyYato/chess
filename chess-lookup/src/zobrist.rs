pub(super) static PIECE_ZOBRIST: [[[u64; 6]; 64]; 2] = [
[[0xa54d55b4883cd5da,0x785dad98ba884535,0xb193d7bbef87f4a8,0xae43c20de7ec12ef,0xbc70ab08b86034d4,0x3fd5d92adf410bf9,],[0x59268386779d67ea,0x96e9342e4505a28f,0x41926471a1f7d953,0xaf0f25895fd742e8,0x9140523e5114080e,0x45e94e2fbbfc1de0,],[0x447775fbebfa8b37,0xc95315c0091ca992,0x6440fc845128dbf3,0xf8b6feb3b889cdfa,0xe9766ac8b64d9208,0x80056d44b35e0192,],[0x91783b1db79e67b4,0xbac4ee05664125e2,0x9d11bde7026bcd40,0x340d79fe29200f6,0x46b744d3415a60d3,0x937e6ea7c52b305,],[0x8bf5f208deca41da,0x47e72594c2e6dd9,0xc68689e26a602769,0x71cf0c27b8524fb5,0x7e252f42fdaa302e,0xf5110301910ce1a6,],[0xc3ee66bcb0139430,0x5ee6516aa0084326,0x915c3ba59d03adfc,0xf1c82ab57a73add6,0xdf0e8ce76fa8f117,0xd0879379a95e4aea,],[0xe9e834262ece9470,0x3c3765f176c213f7,0x6e44c8c5275b8144,0x351077f7dd9642a5,0xa70b4989f85c9916,0x398b71b8bea15c54,],[0xa9a0158c5e756950,0x1e2d48579cac23e6,0x579436e30e95bdae,0x1dea970880ea6a3d,0x7fa24273e04cb348,0x46aaf24a88f2977,],[0xa8a9e7d463d2c9e5,0xa67c8464f9c954b9,0x52009cd303febb8f,0x541069296d08f019,0x8c04eb8e683ea4c2,0x4ee310c9580a9a68,],[0x173e6e1e13b0d928,0x87b88cd51c0267fe,0x4ba1fe2caf2e2b5d,0x5dce2cd36a93bcac,0x4b7c881f650ee23b,0xc7bb543deeeacc98,],[0x842a55303b9cfec1,0x4a6710497861d14,0xb6f5e695fb57d197,0x7813bc635dc95c8c,0xd6a89b158b422a02,0xfd6e8f1aa6cbe2c6,],[0xd7fb0d374561cf05,0x3b67157a75c90278,0xf441f74fdd1e611a,0x5cf50874ed0b25e9,0x119f9638e0839820,0xbc6bef2810be9e2c,],[0xee91b5fee5373470,0xd55739fcfbac02e,0x76150883eeebaae8,0x6a50fc83db79bea0,0xd9f37fdf2f187229,0x63e89752d3183ccf,],[0x5370c1a416ff0fc5,0xe3505e7838f917e8,0xe4719ca556beed49,0xa5e1b7127e35fa11,0x933a87dadc97e56c,0x6f17d0dc1e1de543,],[0x13038569bfdaef81,0x3ea4ac1724459f58,0x843c18a9cbef8d2d,0xd1db9b65bf52f08c,0xf840acc0ec4a5988,0xfaf633970e4cb7fc,],[0x2bcddbc28d81e78e,0x25218b5620e31786,0x6cb00151a99ffa13,0xa920bee2d92c1335,0x2aa4a3210ae0a6cd,0x613b87b3cfcaf1db,],[0x510eec799689c74,0xc8fe408eb515ccb2,0xc7711410152f376b,0x8886b7a803f591a1,0x74fd524f21ec9c0a,0x6ae4273e10c9421c,],[0x8cba1decbee6f98f,0xfb08d6909339917f,0x8569d0e58914a738,0x54a9434e0f7ad182,0xd4cd4d537d19fd12,0x106d249b7910f3ac,],[0x1e5c263515e72720,0xecc93ec98e7f8f1b,0xeb894a375d3e8907,0x6e7e802684470ef7,0xbc647065047e2fb,0xca15d3e52bbe2f77,],[0x1029d53f641b583,0xe30dbeceaac03603,0x3689951fc2eedec2,0x376d32f78cf6f7d4,0x95e6a4ce227787f7,0xb801aa937359147b,],[0xddd90e630b1e1920,0xdca794bdd8214074,0x7471ba9ff1e90f73,0xdcb4b2ef2d594e2b,0xd7d3ee5218086161,0xf1b66b055732d93a,],[0x11ba85fe262d48a6,0xc5f5000442e3218b,0x37402b3c7a9246dd,0xdb8150c5d92b4524,0x59bd61c7deb06813,0x48b0f8e642c2cf0c,],[0xc7685ded00210cc6,0x7d5faf800eb15e43,0x726a8aff7685f47c,0x77ecde99b49678bb,0x1a6a872ea08bbb99,0xdcec881d8e692215,],[0x1d802f81f4bbab89,0x60118e332c6ddfb7,0x5f5ad49081af3d67,0xa276b17973bbf004,0xfe1ddf4d1e27c7aa,0x8338f479f9831c94,],[0x4a1ae31b137b5377,0x99c818bd1a8644d6,0xbc0421defe0bbf97,0xc02127f5e6051d97,0xa5be74a1daecf697,0x7eb03a577261f554,],[0x38011847f62cca49,0x5711cad35999f158,0xf07f72aebd56b78a,0x2b42a3106111177f,0x13135a4b25ac3a1a,0x217e593001a0213e,],[0x7d71959ee0a67e41,0x10d80e2254237426,0x4970a7c4cf402bbc,0x2e7af8e823db6553,0x8bf890256d6a53a1,0x65b8810155139537,],[0x8aef49672b9f30b9,0xe32d3b0fca283e3c,0x31d73548c689faea,0xc8cd2977888cc3fa,0xb07c8f1e31102212,0xa69d8aa1567a67d5,],[0x7faba5952f5b0c44,0x8f4c4b7fb64f2dcc,0xc0ce5ba6ab8134ee,0x9ceec45940b609dc,0xd835328c4e168600,0x58060a0e31cd1107,],[0x77acd87091175e67,0x37834992fd78d633,0x62dac263988e7a8b,0x2ccb77221555275d,0x89b4c0c9d4e4ded0,0xbd145f0e3ca06a1a,],[0x4d39f5c4e2062f30,0xaac5b016390103ed,0x9af9a4d21ac236a1,0xcd1d05fbee26ac89,0xa844398b6f1ec9c1,0xd2dc7f1be9842d43,],[0x8310050b0026126a,0xb7e316b4fe229030,0xe741a9f137662f31,0x3785b1cf7104ac35,0x694b6946eb5498d3,0x82adb8b91d44a469,],[0x4bbbbfddb3db5770,0x63d495fa68e6182e,0xef9b25b8a5f2501a,0xf5a9608d6c5ba9d0,0x5396abe8bf372d8b,0x9b2159083b2ec48b,],[0x3f32c9f8019b12aa,0x8d625674f6da7280,0xf16671c81c1e0a6e,0xc7c7a03716dd1c97,0x7ce16e3535bb1e29,0x15c4fb8190eb8227,],[0x7015357119dfe93c,0x590ae4b81f8131fe,0x3758213283ba4815,0x29787084640cb506,0xf21436ebe122f92e,0x3c00a242bc2ce6ad,],[0x211430c2d9cec3bb,0x86c2e6369f38703,0x92b97a303e7e47fc,0x297972963a85c7e0,0x4b8fb67bcf6ce642,0x422784f17e4c01ea,],[0xeccf045c7dc6a516,0xebcd552c01ae2f34,0xab8067137d2dc315,0x77bf54e0c6e66d97,0x8074be9952972d2b,0xdb052ddf82a4553c,],[0x441ae248257b9eb1,0xf4016090c3ce5e8f,0x12302b2549d51c93,0xafece20ed5c02818,0x645edbb7a3fd987,0x57faf7ad574f6852,],[0x505b8e8a8461918,0x3098f700c1f6795,0x904b4db5cf66d6b1,0x6823bbde0a418fdd,0xc8c071383f572e47,0x3a45f5a7d9ef924d,],[0x89c0dbc6b8d026fe,0x97199dda45b0fb3e,0x43f442637d4e4f37,0x2c8d2622ffe0ad8,0xb2ceeea947c5df5d,0x76a3b6e53528ec3,],[0xf88409eb2ab33aed,0xd9520138a70d7349,0x575187788fb9d142,0x7008e7cc299fe059,0x9cb735b9a8d80fab,0x5d3c4da6f023d1ec,],[0xa44299f08ca24cd2,0xb5afa8b9bed542a4,0xca7e515bb507e508,0x833872e7a331738f,0xe5818fa6a0ca7608,0x2b5e3956f5f188ab,],[0xfc8dd77ab3f077db,0xa06613101f9c1e6c,0x9ab55b8d29ab4618,0x29650ff21e6e29e8,0x43707933df3e2da6,0x958cbe37a3573993,],[0x229c6bf8fe0d46e4,0x1fb88162b6f43fbf,0x561ed125bea311a5,0x6a20081d0e4ef816,0x158597b38b45a067,0xe9b6041776d76501,],[0xa5480249671494a2,0x686133800e6ef883,0x3da313bdac5f84e3,0x997474312f077e07,0x348ee612008b245c,0x75feec4173bcae6b,],[0xaea253aff8e9f791,0x3881b76d8ea26363,0xf9eb750a6ef69aee,0x813028f52ae2aa2f,0xba33ed54f238070a,0x6e5b7fa544e2facd,],[0xdebbb2be2bc4bb22,0xf416bfa86306a8ac,0x7678df7c7e314ffc,0x704ebb5d17473829,0x87dd96af069a78fd,0x4f14fd756e51369f,],[0xcecc4731d8b33e70,0x37c18d8d17801a39,0xe8300cdba59c6029,0xcaa8fc04d18a738c,0x57379d66331b7fc8,0xbcb77845518bc65f,],[0x5022d94f4dd04ba3,0x6a41aa472ecf90b8,0x97a4d5ad0465a587,0xacf351594a85715d,0xeb756f63fceaf890,0x6486437e94711279,],[0x7de0165c83039151,0x3a62908a05b95c4f,0x1e8cc8192692e1b3,0xab6432966456344b,0x33f55899b9cfbfc4,0x42ac2526e8733848,],[0x8bf35776047605f4,0x4b6d148737ab8bf3,0x25d30d2b0b85b522,0x6fec6ef66fb7021,0x9f6271e4a3641572,0x4f40a63f2868fc2f,],[0x3ef5dad463b39683,0x1ec573d7a3506be5,0x9d2c89cce997338,0xc7bda6e2e9270325,0x7ed3cbb2e0edd7d8,0x553ab035b6816e8e,],[0x78fbed035eda664d,0x303f740bc9e58c0b,0xafcf1e819cefcafe,0xf1f6583267cabc1c,0x2365741b60c88ae0,0x8ebba30f223ddd64,],[0xfe602e8d89cecefa,0xad67987dd767a54a,0x529d850eeab8d2e6,0x563d7fb9b8aadba7,0x7bcfd0028415706d,0x4bee33a6990328ec,],[0xd21466bf99082648,0xa977a595fa702ff7,0x70eca7682788df3b,0xc99a0ff3f1284c72,0x35ba3e389ab5c5bf,0xbef6a7a6af639300,],[0x119e40283b46779e,0x23ead265603bee5,0x1d28087ba0720bad,0x1d4ae006a0f626b9,0x8e9d5a4d5ec4f95f,0xccd439368cc6726a,],[0xd465fd703591c641,0xe560f152be213d34,0xf9c0ea72ee052a76,0x42d2ea42be1b92c8,0x4bda837e97738ea5,0x6ecaa8c871bae91c,],[0xc1541b9240e7081c,0x14f6538f4fea9eb6,0x16368c17913b357e,0xe46a93a43a28fd74,0x9e2dcce5722e3a10,0x6bb7c34d380e1b07,],[0x8b4547677cf05509,0x8aae1c5fb6448870,0x79ffcc354cea508f,0x2be498b083839152,0x194946809ab6410f,0x1483d2baf94d33a9,],[0xa13f966a5e873bdf,0xe5858833992ae838,0x438620372836561b,0x175b48ec02d114dc,0x2dda3460f2b2bba0,0x340432a40d6e0856,],[0x5edcd934c70eff0d,0x588117b19d660223,0x1015955350306215,0xacef4a3153aaa046,0x8e7b7743cd18eae,0xd665471084faa031,],[0xc3b978bae3876a7d,0x234b7fe8fbc60fd2,0xd4343b473d9c7adf,0xb6dbd1cd679a7a20,0x53a19b7aefed6a7,0x9aa1174d083b19ca,],[0x50db83cca752b9e6,0xbd26a57993fa2712,0xe6086dc1d9c47f19,0xc8fe12639e3ef76a,0x59d3424d38cc13da,0xa2221bd6b859dc7e,],[0xc4952384a081b681,0x237bfd49e7a8852b,0x4c4e6c0abc13c8c7,0xfe02a83a862a1156,0x16b6fbc2cb6b4bd7,0x24b2ee36396c49d1,],],[[0x57a3616d47a26a14,0xd5a1e3befba2c613,0x6f414ba11a62e4b9,0xcc92adb9724d0e83,0x1bfbb73e21fb4168,0xa8a79198936c5a1a,],[0xae75dcfedfc6cf1f,0x631a92a70765b729,0x55ba688ffe738061,0xa344d233bbc2c298,0x84e39749e3b27de8,0x8d07aa82f63de65c,],[0xcd2a3bbb94c9e80e,0xcc7d0eca98a9d88e,0x364f96d1c3326cd3,0x2bb8b10ecf682efd,0x6826a6e37be36fd6,0x8f85da4bffc397cb,],[0x287913bfc8ec271d,0x3f30721d567c53ab,0x67a40bc1526ee20c,0x592148f13eefbddd,0x5b547ab613665d9f,0x1c4209383c183221,],[0xe75341f93bc18030,0x7246b09b8bf291db,0x814b2bc2c9385eb5,0xcb0aebe5b50d2ca2,0x383bc30e2f37069c,0x1bb23a81204aca43,],[0x38206fc419c34119,0x835aaf6a8d33a4d0,0xff8a0ed7600004d8,0xc54a05b0e7d7c6df,0x36c3dfdc82ea364,0x4967a44c3f4e8930,],[0xb20b7f9ea806af35,0xed01787b7df0d103,0xf59c94ad39707e2c,0xab4c9f2439087d85,0x97b023ddd7d32544,0xea9ddcfede14a6b8,],[0x6bc65d907377e3f7,0x3bb912bcf7818f81,0xe52d5e99c15d7201,0xd0e3904b85b5e5b1,0xe896ccdae3af5d27,0x1552ecc29bf4a716,],[0xbfc6351bd1edb283,0xd79eb835e4d3a839,0x6b6777b5ed3682f3,0xad8e05d4a57ca8d0,0xd860ef4dcd104ee7,0xc1c48436d6d82c57,],[0xd75a70160d637ba,0xa3505308964e7c1c,0xfdb37ddc1e012bd9,0x9b7f67a685ee83ec,0xea55f96f8e31834a,0x7d4913261937e62d,],[0xa4e3d3642d90d008,0x329f96de5bd2fa4a,0x6bc849fd5b1a195b,0x8a646bbeaef8c55f,0x6f79960a021c796d,0x2464957ef1dc8213,],[0xf77408dc8d207e4e,0xf01ed19a7249af1d,0x67db4fe50ad7e58f,0x19ea77b5650bbc49,0x2f9962e50ff18770,0xb2b90d895d60b284,],[0x3babd6b038768bfb,0x4066e3d5f3886b6a,0xac3afaad4c725388,0x7dfc5ac5d18dde1f,0x70965f7b8dfd27b7,0x36fb20fb4e7bba5,],[0x88a6f55f66123feb,0xaab9e7a04f246995,0xf409d5d43e727a9e,0x2e14847b5dc3545a,0x1ed8ea74059e3889,0xed0085ca54af0bf4,],[0xb742488a8b75b0b9,0x7153f3693a5c8df3,0x93706c77c872a6b9,0x6bafe6c79d9f1be3,0x58fd902e00de3371,0xc9ba456aa29f32c8,],[0x1ccd605e0e99b8b5,0x72cc949dc3d73394,0xe87bb28b168b0bb7,0xdcec35f955f85abb,0x21f0309365741fc8,0x2e6d18695ad339f5,],[0x9c13de793a43a496,0xd8c9e34d1e1406bd,0xd04b3dd112a828b5,0x6c3f9365589588f4,0xc49a8fadf74ab4b5,0xf42b98eede77087b,],[0x3bc9d29cf85e67f8,0x6309579388dc08c7,0xe1da0a949c170f6d,0x45ab37a237767a56,0x9c6aa3fed08568b5,0xdf356efd04b7f816,],[0xc6c8e0210457f67,0x89a848838bf2dccb,0x945f35522f3e6140,0x1234ed84a331fff5,0xabd49dc4528212b5,0xb1d91f6daf6ce63a,],[0x96d1a72d25ef1ec8,0xc89d3afaf6046317,0x1d13530646ac9a0a,0x472611ba5f8e9ce6,0x9199f025f75b2f41,0xd2bc61511d469f69,],[0xb8b5d6f503b67ed2,0x715e08678a44f970,0x495d6870fecc09e1,0x1cd36aee17739745,0xcae54d568b090223,0x25cf506c048d46a7,],[0x245a460888e81ee1,0x8e8a6c6a05da6e9f,0x2012b21ef97ee1f,0x658114869d16be0f,0xfa2565d62f514614,0x9a0976dbbe1c4cac,],[0x83ca1d3e206c7bd6,0x3695530c562cb0e,0x4ab69a9e3ccfd5ae,0x9590898e5b8eee54,0x3e970aeaddf63dd9,0x73535bae24567d65,],[0x5c3575dbadf61768,0xc1b5d924daf2b575,0x572d3695696dc072,0x7619a97ed0a22685,0xfe0774992455b677,0x5699b5ac68975a88,],[0x628aa58b69f31ced,0xc6699db6da13e399,0x706ab10fa34cdebe,0xc8f46994de657995,0x1e2f6c1e2a12afee,0x3ea3f74b2e90b2d9,],[0x5b6f4d79c18a6b75,0x10418abfecc9c8a6,0xab6e56ddf26f96dd,0xee037012cf5e8ffc,0x825c418e71d18f07,0x10a0ebe038b289d1,],[0xbe46725669251024,0xe78b13700e002c81,0xcea94adcc1290883,0x889ad46ae6317790,0x9cfb5c4a4ef1285f,0x535788073b548e47,],[0x9dca19e7b6b5853c,0xcc9a1cb786d9903f,0x15166ff4ff48cc27,0x788c3fab7ccbf2aa,0x367eff26efd9dd22,0x2d3ae284ff25da8,],[0x7ff682fdf5c5cc9b,0x92c05a13e1f32633,0xdb3352a6969a80a9,0xf08f61f62fa1234f,0x36c5b4bfc5271a52,0x8db1624acee4e15f,],[0x7ba66b9b748b4c58,0x178afe7d9e550eff,0x5344815a935c3372,0x5f623de74fbf0c8,0x1c0efcc8a5e0ff9,0xf8223697e0595eb1,],[0x4887f928b30ed08c,0x57775afdd7232e1b,0x5fca8258ac5f3009,0xe211a00948283867,0xc3b6282a8fc3a143,0x9b41eadd00fcef02,],[0xbe50df0313fa39be,0x69a06f6c1dea7951,0x9e0e6873266a0642,0x414cbf338b59c235,0x8e3e5360415c0839,0x6f6f97c621c49787,],[0x1878ca13c6be4c31,0xd143e218253e7bfd,0x1fb6b81e549b10e7,0x881cdf2b14abae89,0x8da7e7b7f17b86d8,0xe7557476c8088bb,],[0xfa92c991308fbe06,0xfa9993e5984f65e8,0xf23d04b788950791,0x8518ef0bbba4fa1a,0xb179c691c162c91b,0xf8b399e28691cc57,],[0x5c67af37977c3ddc,0x8e43030ae7580d7,0x74eb2afb3c1f779,0xa05706025fc4aecd,0xf68804949d40e1b6,0x6894f776158818e4,],[0xf148d3d384dddaa,0x19be7385b22f1477,0x4fb833d9afafb0e9,0x80243da64e24b2a,0xdcc57d389be91f11,0x8b05ccafc74ab05f,],[0x2fcfaa0ba919c233,0x1546ec101af287eb,0xb92d1558d0224bc7,0x1055e9b9e604ea9a,0x8e3c6dab4cb4c910,0x51a0957ab8c569ae,],[0xdfec4736bee27c32,0x12feea41eeadd541,0xa6f59c3cfc8c1452,0x72b7e986b958f645,0x8293fc399dd6914e,0x686dcfc7c02a349c,],[0x9392637169e4956a,0xc5767f695e611650,0xee9c096b1f900b0,0xbd61b0da9dd7fea7,0x880672030b78acb1,0x1f75d44ad7f92526,],[0xc46a1eefef65ddfb,0x57ea3d8a64ecc2b8,0xc0f9496ef25fc83f,0xa60003f616b1ad98,0xfc419d932db18e51,0x8194141d40e0a730,],[0xbb54aa8d924b67b5,0x340e75e2eda71240,0x3fcce16a15269f3c,0x114f667b3357593d,0x382f75083a168fb2,0x94ada642dfabf8ae,],[0xde6f284f32829f25,0x4092ea2784e05b24,0xff3968f75db69aa9,0x604d2855e8acdbd9,0x494b308466201ed4,0x82c33ad6602896f7,],[0xa7a67f41388be164,0xdbe32e5f0f450168,0xe2451a0bab944b47,0x25812851c6152987,0xf61b4bf89bbce8ae,0x8bc476ce28b128fd,],[0xed1573ee72c8f2f2,0x1c5a524e5f1fdd3,0x226a0d3962b08918,0xaf4cbdc1ec876764,0x9f0bff5aa48fc531,0x119fa9ac232eb908,],[0x8051d7f99f28b097,0x73b90659fda37b33,0x3afb8bd031fd5dc4,0x3b32406ce34323b0,0x5a9888bc8d31e390,0xb11400f32737743b,],[0xb9fa6971e1b29094,0x79569c8066904011,0xcede693154b8b4b7,0xbc9ab402afb9499c,0x6a543781c3cc1d55,0x50aabd976f95fd49,],[0x55a1a6b6b648a7fc,0x9ebd2c3244a2faed,0x864e687c511323e8,0xa8cf2ac102174b78,0x3665ea8026a22e00,0x8434d5c95b0750c0,],[0xf1fbcfe39623af2b,0x25f1430eb0980f5e,0xd50a11789e0808b2,0x57e3594722bd2ab0,0xeeeeeb5ffa9d10a8,0x63678ab4c74e4f46,],[0x5e500a9a0216aaa9,0x16bf5ee58d152c9,0xa243797e982a8771,0x3c4c4ad8b00cc6d8,0x69525467ef3e269f,0x3151ae97847c1545,],[0x757526ce2321a6d9,0xb8b6c07235d3d9e6,0xde7ce978177c2011,0xb9f5dcbe38b35a1f,0x380d2072eb1da4fe,0x55e5e5a621868184,],[0x37ba55b0360ee22b,0x9b8aca59681a97d5,0xbb704bd91c91706,0xc5a56ebc157ac6a0,0x14607f7f0acc18dd,0x548c10a3b5fbfef5,],[0x4a1829a0cfab958b,0x5694a96799e908c0,0xedb13423ba8981fe,0xf96f24cee86443e8,0xeb88baee38fe8db,0x8759f221ab50746c,],[0x3c7f7cfb98213a1,0x284ac41f6c551f4b,0xc91bac2bf59ff97e,0x8a3fd7f08a4db3ae,0x384c2482989cc6b1,0xd55c1d844278a53a,],[0x8b0b6caa803ca235,0x3211c31674fc521a,0x8e282ee72c72a41c,0xa267019cbbc66fd2,0xba1c4f6f08b57db5,0xe036a81ec760e888,],[0x18671c1b4d03cca6,0x693b14478199891b,0xaa1cac86549abea0,0x4330070a15b30ec4,0x186c7f624cc556f8,0xfd32a95c5a76e477,],[0x4730811f741eac53,0x620617fab1b7531d,0x3a72035a51ebbe26,0x3fa32ae6eb54efb4,0x5cc93d59b79f7846,0x4093132e4db9ca67,],[0x9349b87b42a233c3,0x4a7c3b8032486ec5,0x2f694f0cc07da433,0xeac34a13f8da547a,0x40de68c3efbca547,0x5b0a80183e30e489,],[0xda8a41a20b58d25c,0xbf86192143d2930e,0x9e6d40459b5f529c,0x47fb983e7b8dcec5,0xa7a93c152a182cab,0xd4f3970dd0f3f4a4,],[0xb5da5f97c6b0650c,0xea61778414bf5009,0x7a4efddc273d4115,0xb61a64c85f150964,0xa788667589e515e6,0xfd48a1c0cc8662e5,],[0x879efd26146c01bb,0x9b9b11f9ffd85b52,0xe7dbbde038b37257,0x36688fac8c7fc5ec,0xfff66f076810def1,0x584463f620c7d441,],[0xd329f79609dfcd52,0x7e16a9c583b8a930,0xab9968e77c3c2870,0x64c9b288ea468216,0x6dcc022973be9dbd,0x962dbb92b18f45d6,],[0x49d307aecb9bf926,0xc2ec9e78e89fcdd9,0x3e89a1b9c0cc7952,0xda6e7aea4d72b9e8,0x5dcbe29434c39c91,0x61a9f12bda60eaea,],[0x83e8dc208d40d922,0x201efe5fb706b166,0xdfb898ee1aa304e7,0xd2f055fa915c08ca,0x4010858dd55b86f2,0x664ee1b6f295527d,],[0x8f943e5c14f83d62,0xbde4056b0439a3ce,0xec60dd585a4de777,0x6d762b35dbb5079b,0xa1dced33c9245caf,0xcfb19970ba4cbae4,],],];
pub(super) static CASTLE_ZOBRIST: [u64; 16] = [
0x220a02aeaa82571c,0xbae82e765944bcb,0x3725351c5a0d4661,0xc0e32fc26c684d9f,0x2617823d4898b703,0x501078eb67ab0476,0x465b3729e3b54178,0x19eab153acd99b1b,0xc98a600d6d5b7d2,0xeb30e7eb491412,0x9e44aa280295d11a,0xe4835b696f9f7086,0x7d20dc142ae08839,0x9af8a298e2dc4a4e,0x22d508feee893c28,0x7c5dac051fe4a2e5,];
pub(super) static EN_PASSANT_ZOBRIST: [u64; 8] = [
0xb9a51955d3a5dbc8,0xaeb3ca93bbf58bbd,0xfcbbd65335ae6708,0xb395918ec18078b9,0xa13acf4c6d25c1a1,0xe99d9f232ac61d7b,0xe77c172113ee7479,0xfac9514b802fa0f8,];
pub(super) static TURN_ZOBRIST: [u64; 2] = [
0xd2289f78fbaf7cb6,0x217d3c9ebfbf1f41,];
