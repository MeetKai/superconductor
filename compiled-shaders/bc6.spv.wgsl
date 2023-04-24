var<private> gl_FragCoord_1: vec4<f32>;
@group(0) @binding(0) 
var uInput: texture_2d<u32>;
var<private> uOutput: vec4<f32>;
var<private> global: vec2<f32>;

fn main_1() {
    var local: vec4<u32>;
    var local_1: array<i32,8u>;
    var local_2: vec4<u32>;
    var local_3: array<i32,8u>;
    var local_4: vec4<u32>;
    var local_5: array<i32,8u>;
    var local_6: vec4<u32>;
    var local_7: array<i32,16u>;
    var local_8: vec4<u32>;
    var local_9: array<i32,8u>;
    var local_10: vec4<u32>;
    var local_11: array<i32,16u>;
    var local_12: vec4<u32>;
    var local_13: array<i32,8u>;
    var local_14: vec4<u32>;
    var local_15: array<i32,16u>;
    var local_16: vec4<u32>;
    var local_17: array<i32,8u>;
    var local_18: vec4<u32>;
    var local_19: array<i32,16u>;
    var local_20: vec4<u32>;
    var local_21: array<i32,8u>;
    var local_22: vec4<u32>;
    var local_23: array<i32,8u>;
    var local_24: vec4<u32>;
    var local_25: array<i32,8u>;
    var local_26: vec4<u32>;
    var local_27: array<i32,8u>;
    var indexable: array<i32,32u>;
    var indexable_1: array<i32,32u>;
    var phi_41503_: vec3<i32>;
    var phi_41491_: vec3<i32>;
    var phi_41492_: i32;
    var phi_41493_: i32;
    var phi_41494_: i32;
    var phi_41510_: i32;
    var phi_41511_: i32;
    var phi_41512_: i32;
    var phi_41521_: i32;
    var phi_41520_: i32;
    var phi_41623_: vec3<i32>;
    var phi_41611_: vec3<i32>;
    var phi_41612_: i32;
    var phi_41613_: i32;
    var phi_41614_: i32;
    var phi_41630_: i32;
    var phi_41631_: i32;
    var phi_41632_: i32;
    var phi_41641_: i32;
    var phi_41640_: i32;
    var phi_41743_: vec3<i32>;
    var phi_41731_: vec3<i32>;
    var phi_41732_: i32;
    var phi_41733_: i32;
    var phi_41734_: i32;
    var phi_41750_: i32;
    var phi_41751_: i32;
    var phi_41752_: i32;
    var phi_41761_: i32;
    var phi_41760_: i32;
    var phi_41851_: vec3<i32>;
    var phi_41839_: vec3<i32>;
    var phi_41840_: i32;
    var phi_41841_: i32;
    var phi_41842_: i32;
    var phi_41858_: i32;
    var phi_41859_: i32;
    var phi_41860_: i32;
    var phi_41869_: i32;
    var phi_41868_: i32;
    var phi_41933_: i32;
    var phi_41932_: i32;
    var phi_42027_: vec3<i32>;
    var phi_42015_: vec3<i32>;
    var phi_42016_: i32;
    var phi_42017_: i32;
    var phi_42018_: i32;
    var phi_42034_: i32;
    var phi_42035_: i32;
    var phi_42036_: i32;
    var phi_42045_: i32;
    var phi_42044_: i32;
    var phi_42081_: i32;
    var phi_42082_: i32;
    var phi_42083_: i32;
    var phi_42098_: i32;
    var phi_42099_: i32;
    var phi_42100_: i32;
    var phi_42109_: i32;
    var phi_42108_: i32;
    var phi_42197_: vec3<i32>;
    var phi_42185_: vec3<i32>;
    var phi_42186_: i32;
    var phi_42187_: i32;
    var phi_42188_: i32;
    var phi_42204_: i32;
    var phi_42205_: i32;
    var phi_42206_: i32;
    var phi_42215_: i32;
    var phi_42214_: i32;
    var phi_42251_: i32;
    var phi_42252_: i32;
    var phi_42253_: i32;
    var phi_42268_: i32;
    var phi_42269_: i32;
    var phi_42270_: i32;
    var phi_42279_: i32;
    var phi_42278_: i32;
    var phi_42371_: vec3<i32>;
    var phi_42359_: vec3<i32>;
    var phi_42360_: i32;
    var phi_42361_: i32;
    var phi_42362_: i32;
    var phi_42378_: i32;
    var phi_42379_: i32;
    var phi_42380_: i32;
    var phi_42389_: i32;
    var phi_42388_: i32;
    var phi_42413_: i32;
    var phi_42414_: i32;
    var phi_42415_: i32;
    var phi_42430_: i32;
    var phi_42431_: i32;
    var phi_42432_: i32;
    var phi_42441_: i32;
    var phi_42440_: i32;
    var phi_42525_: vec3<i32>;
    var phi_42513_: vec3<i32>;
    var phi_42514_: i32;
    var phi_42515_: i32;
    var phi_42516_: i32;
    var phi_42532_: i32;
    var phi_42533_: i32;
    var phi_42534_: i32;
    var phi_42543_: i32;
    var phi_42542_: i32;
    var phi_42794_: vec3<i32>;
    var phi_42791_: vec3<i32>;
    var phi_42788_: i32;
    var phi_42637_: vec3<i32>;
    var phi_42625_: vec3<i32>;
    var phi_42626_: i32;
    var phi_42627_: i32;
    var phi_42628_: i32;
    var phi_42644_: i32;
    var phi_42645_: i32;
    var phi_42646_: i32;
    var phi_42655_: i32;
    var phi_42654_: i32;
    var phi_42757_: vec3<i32>;
    var phi_42745_: vec3<i32>;
    var phi_42746_: i32;
    var phi_42747_: i32;
    var phi_42748_: i32;
    var phi_42764_: i32;
    var phi_42765_: i32;
    var phi_42766_: i32;
    var phi_42775_: i32;
    var phi_42774_: i32;
    var phi_42795_: vec3<i32>;
    var phi_42792_: vec3<i32>;
    var phi_42789_: i32;
    var phi_42793_: vec3<i32>;
    var phi_42790_: vec3<i32>;
    var phi_42787_: i32;

    let _e176 = gl_FragCoord_1;
    let _e178 = vec2<i32>(_e176.xy);
    let _e181 = (_e178 & vec2<i32>(3, 3));
    let _e185 = ((4 * _e181.y) + _e181.x);
    let _e186 = textureLoad(uInput, (_e178 >> bitcast<vec2<u32>>(vec2<i32>(2, 2))), 0);
    let _e191 = bitcast<i32>(((_e186.x >> bitcast<u32>(0)) & 31u));
    let _e196 = bitcast<i32>(((_e186.z >> bitcast<u32>(13)) & 31u));
    indexable = array<i32,32u>(52428, 34952, 61166, 60616, 51328, 65260, 65224, 60544, 51200, 65516, 65152, 59392, 65512, 65280, 65520, 61440, 63248, 142, 28928, 2254, 140, 29456, 12544, 36046, 2188, 12560, 26214, 13932, 6120, 4080, 29070, 14748);
    let _e198 = indexable[_e196];
    let _e201 = ((_e198 >> bitcast<u32>(_e185)) & 1);
    indexable_1 = array<i32,32u>(15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 2, 8, 2, 2, 8, 8, 15, 2, 8, 2, 2, 8, 8, 2, 2);
    let _e203 = indexable_1[_e196];
    if ((_e191 & 2) == 0) {
        if ((_e191 & 1) != 0) {
            let _e3899 = vec3<i32>(bitcast<i32>(((_e186.x >> bitcast<u32>(5)) & 127u)), bitcast<i32>(((_e186.x >> bitcast<u32>(15)) & 127u)), bitcast<i32>(((_e186.x >> bitcast<u32>(25)) & 127u)));
            if (_e201 != 0) {
                let _e3926 = bitcast<i32>(_e186.z);
                let _e3946 = bitcast<i32>(_e186.x);
                phi_42757_ = (vec3<i32>(((((_e3926 >> bitcast<u32>(7)) & 63) << bitcast<u32>(26u)) >> bitcast<u32>(26u)), (bitcast<i32>(((_e186.y >> bitcast<u32>(19)) & 15u)) | (((((_e3946 >> bitcast<u32>(3)) & 3) << bitcast<u32>(30u)) >> bitcast<u32>(30u)) << bitcast<u32>(4))), ((((bitcast<i32>(((_e186.x >> bitcast<u32>(12)) & 3u)) | (bitcast<i32>(((_e186.x >> bitcast<u32>(23)) & 1u)) << bitcast<u32>(2))) | (bitcast<i32>(((_e186.y >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(3))) | (bitcast<i32>(((_e186.y >> bitcast<u32>(2)) & 1u)) << bitcast<u32>(4))) | (((((bitcast<i32>(_e186.y) >> bitcast<u32>(1)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(5)))) + _e3899);
                phi_42745_ = (_e3899 + vec3<i32>(((((_e3926 >> bitcast<u32>(1)) & 63) << bitcast<u32>(26u)) >> bitcast<u32>(26u)), ((bitcast<i32>(((_e186.y >> bitcast<u32>(9)) & 15u)) | (bitcast<i32>(((_e186.x >> bitcast<u32>(24)) & 1u)) << bitcast<u32>(4))) | (((((_e3946 >> bitcast<u32>(2)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(5))), (((bitcast<i32>(((_e186.y >> bitcast<u32>(29)) & 7u)) | (bitcast<i32>(((_e186.z >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(3))) | (bitcast<i32>(((_e186.x >> bitcast<u32>(14)) & 1u)) << bitcast<u32>(4))) | (((((_e3946 >> bitcast<u32>(22)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(5)))));
            } else {
                let _e3902 = bitcast<i32>(_e186.y);
                phi_42757_ = (vec3<i32>(((((_e3902 >> bitcast<u32>(3)) & 63) << bitcast<u32>(26u)) >> bitcast<u32>(26u)), ((((_e3902 >> bitcast<u32>(13)) & 63) << bitcast<u32>(26u)) >> bitcast<u32>(26u)), ((((_e3902 >> bitcast<u32>(23)) & 63) << bitcast<u32>(26u)) >> bitcast<u32>(26u))) + _e3899);
                phi_42745_ = _e3899;
            }
            let _e4047 = phi_42757_;
            let _e4049 = phi_42745_;
            let _e4050 = bitcast<vec3<u32>>(_e4049);
            let _e4064 = bitcast<vec3<i32>>(vec3<u32>(((_e4050.x >> bitcast<u32>(0)) & 127u), ((_e4050.y >> bitcast<u32>(0)) & 127u), ((_e4050.z >> bitcast<u32>(0)) & 127u)));
            let _e4069 = (((_e4064 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(6, 6, 6)));
            let _e4070 = (_e4064 == vec3<i32>(0, 0, 0));
            if _e4070.x {
                phi_42746_ = 0;
            } else {
                phi_42746_ = _e4069.x;
            }
            let _e4074 = phi_42746_;
            if _e4070.y {
                phi_42747_ = 0;
            } else {
                phi_42747_ = _e4069.y;
            }
            let _e4078 = phi_42747_;
            if _e4070.z {
                phi_42748_ = 0;
            } else {
                phi_42748_ = _e4069.z;
            }
            let _e4082 = phi_42748_;
            let _e4083 = (_e4064 == vec3<i32>(127, 127, 127));
            let _e4091 = bitcast<vec3<u32>>(_e4047);
            let _e4105 = bitcast<vec3<i32>>(vec3<u32>(((_e4091.x >> bitcast<u32>(0)) & 127u), ((_e4091.y >> bitcast<u32>(0)) & 127u), ((_e4091.z >> bitcast<u32>(0)) & 127u)));
            let _e4110 = (((_e4105 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(6, 6, 6)));
            let _e4111 = (_e4105 == vec3<i32>(0, 0, 0));
            if _e4111.x {
                phi_42764_ = 0;
            } else {
                phi_42764_ = _e4110.x;
            }
            let _e4115 = phi_42764_;
            if _e4111.y {
                phi_42765_ = 0;
            } else {
                phi_42765_ = _e4110.y;
            }
            let _e4119 = phi_42765_;
            if _e4111.z {
                phi_42766_ = 0;
            } else {
                phi_42766_ = _e4110.z;
            }
            let _e4123 = phi_42766_;
            let _e4124 = (_e4105 == vec3<i32>(127, 127, 127));
            let _e4137 = max(((81 + (_e185 * 3)) - select(0, 1, (_e185 > _e203))), 82);
            let _e4141 = select(3, 2, ((_e185 == 0) || (_e185 == _e203)));
            local = _e186;
            if (_e4141 <= 0) {
                phi_42774_ = 0;
            } else {
                let _e4148 = (_e4137 >> bitcast<u32>(5));
                if ((((_e4137 + _e4141) - 1) >> bitcast<u32>(5)) == _e4148) {
                    let _e4179 = local[_e4148];
                    phi_42775_ = bitcast<i32>(((_e4179 >> bitcast<u32>((_e4137 & 31))) & bitcast<u32>(((1 << bitcast<u32>(_e4141)) - 1))));
                } else {
                    let _e4150 = (_e4137 & 31);
                    let _e4151 = (32 - _e4150);
                    let _e4153 = local[_e4148];
                    let _e4165 = local[(_e4148 + 1)];
                    phi_42775_ = (bitcast<i32>(((_e4153 >> bitcast<u32>(_e4150)) & bitcast<u32>(((1 << bitcast<u32>(_e4151)) - 1)))) | (bitcast<i32>(((_e4165 >> bitcast<u32>(0)) & bitcast<u32>(((1 << bitcast<u32>((_e4141 - _e4151))) - 1)))) << bitcast<u32>(_e4151)));
                }
                let _e4189 = phi_42775_;
                phi_42774_ = _e4189;
            }
            let _e4191 = phi_42774_;
            local_1 = array<i32,8u>(0, 9, 18, 27, 37, 46, 55, 64);
            let _e4193 = local_1[_e4191];
            phi_42795_ = vec3<i32>(select(_e4074, 65535, _e4083.x), select(_e4078, 65535, _e4083.y), select(_e4082, 65535, _e4083.z));
            phi_42792_ = vec3<i32>(select(_e4115, 65535, _e4124.x), select(_e4119, 65535, _e4124.y), select(_e4123, 65535, _e4124.z));
            phi_42789_ = _e4193;
        } else {
            let _e3608 = vec3<i32>(bitcast<i32>(((_e186.x >> bitcast<u32>(5)) & 1023u)), bitcast<i32>(((_e186.x >> bitcast<u32>(15)) & 1023u)), (bitcast<i32>(((_e186.x >> bitcast<u32>(25)) & 127u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(0)) & 7u)) << bitcast<u32>(7))));
            if (_e201 != 0) {
                let _e3634 = bitcast<i32>(_e186.z);
                let _e3646 = bitcast<i32>(_e186.x);
                phi_42637_ = (vec3<i32>(((((_e3634 >> bitcast<u32>(7)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), (bitcast<i32>(((_e186.y >> bitcast<u32>(19)) & 15u)) | (((((bitcast<i32>(_e186.y) >> bitcast<u32>(8)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4))), ((((bitcast<i32>(((_e186.y >> bitcast<u32>(18)) & 1u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(28)) & 1u)) << bitcast<u32>(1))) | (bitcast<i32>(((_e186.z >> bitcast<u32>(6)) & 1u)) << bitcast<u32>(2))) | (bitcast<i32>(((_e186.z >> bitcast<u32>(12)) & 1u)) << bitcast<u32>(3))) | (((((_e3646 >> bitcast<u32>(4)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4)))) + _e3608);
                phi_42625_ = (_e3608 + vec3<i32>(((((_e3634 >> bitcast<u32>(1)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), (bitcast<i32>(((_e186.y >> bitcast<u32>(9)) & 15u)) | (((((_e3646 >> bitcast<u32>(2)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4))), ((bitcast<i32>(((_e186.y >> bitcast<u32>(29)) & 7u)) | (bitcast<i32>(((_e186.z >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(3))) | (((((_e3646 >> bitcast<u32>(3)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4)))));
            } else {
                let _e3610 = bitcast<i32>(_e186.y);
                phi_42637_ = (vec3<i32>(((((_e3610 >> bitcast<u32>(3)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), ((((_e3610 >> bitcast<u32>(13)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), ((((_e3610 >> bitcast<u32>(23)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u))) + _e3608);
                phi_42625_ = _e3608;
            }
            let _e3740 = phi_42637_;
            let _e3742 = phi_42625_;
            let _e3743 = bitcast<vec3<u32>>(_e3742);
            let _e3757 = bitcast<vec3<i32>>(vec3<u32>(((_e3743.x >> bitcast<u32>(0)) & 1023u), ((_e3743.y >> bitcast<u32>(0)) & 1023u), ((_e3743.z >> bitcast<u32>(0)) & 1023u)));
            let _e3762 = (((_e3757 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(9, 9, 9)));
            let _e3763 = (_e3757 == vec3<i32>(0, 0, 0));
            if _e3763.x {
                phi_42626_ = 0;
            } else {
                phi_42626_ = _e3762.x;
            }
            let _e3767 = phi_42626_;
            if _e3763.y {
                phi_42627_ = 0;
            } else {
                phi_42627_ = _e3762.y;
            }
            let _e3771 = phi_42627_;
            if _e3763.z {
                phi_42628_ = 0;
            } else {
                phi_42628_ = _e3762.z;
            }
            let _e3775 = phi_42628_;
            let _e3776 = (_e3757 == vec3<i32>(1023, 1023, 1023));
            let _e3784 = bitcast<vec3<u32>>(_e3740);
            let _e3798 = bitcast<vec3<i32>>(vec3<u32>(((_e3784.x >> bitcast<u32>(0)) & 1023u), ((_e3784.y >> bitcast<u32>(0)) & 1023u), ((_e3784.z >> bitcast<u32>(0)) & 1023u)));
            let _e3803 = (((_e3798 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(9, 9, 9)));
            let _e3804 = (_e3798 == vec3<i32>(0, 0, 0));
            if _e3804.x {
                phi_42644_ = 0;
            } else {
                phi_42644_ = _e3803.x;
            }
            let _e3808 = phi_42644_;
            if _e3804.y {
                phi_42645_ = 0;
            } else {
                phi_42645_ = _e3803.y;
            }
            let _e3812 = phi_42645_;
            if _e3804.z {
                phi_42646_ = 0;
            } else {
                phi_42646_ = _e3803.z;
            }
            let _e3816 = phi_42646_;
            let _e3817 = (_e3798 == vec3<i32>(1023, 1023, 1023));
            let _e3830 = max(((81 + (_e185 * 3)) - select(0, 1, (_e185 > _e203))), 82);
            let _e3834 = select(3, 2, ((_e185 == 0) || (_e185 == _e203)));
            local_2 = _e186;
            if (_e3834 <= 0) {
                phi_42654_ = 0;
            } else {
                let _e3841 = (_e3830 >> bitcast<u32>(5));
                if ((((_e3830 + _e3834) - 1) >> bitcast<u32>(5)) == _e3841) {
                    let _e3872 = local_2[_e3841];
                    phi_42655_ = bitcast<i32>(((_e3872 >> bitcast<u32>((_e3830 & 31))) & bitcast<u32>(((1 << bitcast<u32>(_e3834)) - 1))));
                } else {
                    let _e3843 = (_e3830 & 31);
                    let _e3844 = (32 - _e3843);
                    let _e3846 = local_2[_e3841];
                    let _e3858 = local_2[(_e3841 + 1)];
                    phi_42655_ = (bitcast<i32>(((_e3846 >> bitcast<u32>(_e3843)) & bitcast<u32>(((1 << bitcast<u32>(_e3844)) - 1)))) | (bitcast<i32>(((_e3858 >> bitcast<u32>(0)) & bitcast<u32>(((1 << bitcast<u32>((_e3834 - _e3844))) - 1)))) << bitcast<u32>(_e3844)));
                }
                let _e3882 = phi_42655_;
                phi_42654_ = _e3882;
            }
            let _e3884 = phi_42654_;
            local_3 = array<i32,8u>(0, 9, 18, 27, 37, 46, 55, 64);
            let _e3886 = local_3[_e3884];
            phi_42795_ = vec3<i32>(select(_e3767, 65535, _e3776.x), select(_e3771, 65535, _e3776.y), select(_e3775, 65535, _e3776.z));
            phi_42792_ = vec3<i32>(select(_e3808, 65535, _e3817.x), select(_e3812, 65535, _e3817.y), select(_e3816, 65535, _e3817.z));
            phi_42789_ = _e3886;
        }
        let _e4195 = phi_42795_;
        let _e4197 = phi_42792_;
        let _e4199 = phi_42789_;
        phi_42793_ = _e4195;
        phi_42790_ = _e4197;
        phi_42787_ = _e4199;
    } else {
        switch _e191 {
            case 2: {
                let _e3330 = vec3<i32>((bitcast<i32>(((_e186.x >> bitcast<u32>(5)) & 1023u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(8)) & 1u)) << bitcast<u32>(10))), (bitcast<i32>(((_e186.x >> bitcast<u32>(15)) & 1023u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(17)) & 1u)) << bitcast<u32>(10))), ((bitcast<i32>(((_e186.x >> bitcast<u32>(25)) & 127u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(0)) & 7u)) << bitcast<u32>(7))) | (bitcast<i32>(((_e186.y >> bitcast<u32>(27)) & 1u)) << bitcast<u32>(10))));
                if (_e201 != 0) {
                    let _e3356 = bitcast<i32>(_e186.z);
                    let _e3364 = bitcast<i32>(_e186.y);
                    phi_42525_ = (vec3<i32>(((((_e3356 >> bitcast<u32>(7)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), ((((_e3364 >> bitcast<u32>(19)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)), (((bitcast<i32>(((_e186.y >> bitcast<u32>(18)) & 1u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(28)) & 1u)) << bitcast<u32>(1))) | (bitcast<i32>(((_e186.z >> bitcast<u32>(6)) & 1u)) << bitcast<u32>(2))) | (((((_e3356 >> bitcast<u32>(12)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(3)))) + _e3330);
                    phi_42513_ = (_e3330 + vec3<i32>(((((_e3356 >> bitcast<u32>(1)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), ((((_e3364 >> bitcast<u32>(9)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)), (bitcast<i32>(((_e186.y >> bitcast<u32>(29)) & 7u)) | (((((_e3356 >> bitcast<u32>(0)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(3)))));
                } else {
                    let _e3332 = bitcast<i32>(_e186.y);
                    phi_42525_ = (vec3<i32>(((((_e3332 >> bitcast<u32>(3)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), ((((_e3332 >> bitcast<u32>(13)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)), ((((_e3332 >> bitcast<u32>(23)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u))) + _e3330);
                    phi_42513_ = _e3330;
                }
                let _e3433 = phi_42525_;
                let _e3435 = phi_42513_;
                let _e3436 = bitcast<vec3<u32>>(_e3435);
                let _e3450 = bitcast<vec3<i32>>(vec3<u32>(((_e3436.x >> bitcast<u32>(0)) & 2047u), ((_e3436.y >> bitcast<u32>(0)) & 2047u), ((_e3436.z >> bitcast<u32>(0)) & 2047u)));
                let _e3455 = (((_e3450 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(10, 10, 10)));
                let _e3456 = (_e3450 == vec3<i32>(0, 0, 0));
                if _e3456.x {
                    phi_42514_ = 0;
                } else {
                    phi_42514_ = _e3455.x;
                }
                let _e3460 = phi_42514_;
                if _e3456.y {
                    phi_42515_ = 0;
                } else {
                    phi_42515_ = _e3455.y;
                }
                let _e3464 = phi_42515_;
                if _e3456.z {
                    phi_42516_ = 0;
                } else {
                    phi_42516_ = _e3455.z;
                }
                let _e3468 = phi_42516_;
                let _e3469 = (_e3450 == vec3<i32>(2047, 2047, 2047));
                let _e3477 = bitcast<vec3<u32>>(_e3433);
                let _e3491 = bitcast<vec3<i32>>(vec3<u32>(((_e3477.x >> bitcast<u32>(0)) & 2047u), ((_e3477.y >> bitcast<u32>(0)) & 2047u), ((_e3477.z >> bitcast<u32>(0)) & 2047u)));
                let _e3496 = (((_e3491 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(10, 10, 10)));
                let _e3497 = (_e3491 == vec3<i32>(0, 0, 0));
                if _e3497.x {
                    phi_42532_ = 0;
                } else {
                    phi_42532_ = _e3496.x;
                }
                let _e3501 = phi_42532_;
                if _e3497.y {
                    phi_42533_ = 0;
                } else {
                    phi_42533_ = _e3496.y;
                }
                let _e3505 = phi_42533_;
                if _e3497.z {
                    phi_42534_ = 0;
                } else {
                    phi_42534_ = _e3496.z;
                }
                let _e3509 = phi_42534_;
                let _e3510 = (_e3491 == vec3<i32>(2047, 2047, 2047));
                let _e3523 = max(((81 + (_e185 * 3)) - select(0, 1, (_e185 > _e203))), 82);
                let _e3527 = select(3, 2, ((_e185 == 0) || (_e185 == _e203)));
                local_4 = _e186;
                if (_e3527 <= 0) {
                    phi_42542_ = 0;
                } else {
                    let _e3534 = (_e3523 >> bitcast<u32>(5));
                    if ((((_e3523 + _e3527) - 1) >> bitcast<u32>(5)) == _e3534) {
                        let _e3565 = local_4[_e3534];
                        phi_42543_ = bitcast<i32>(((_e3565 >> bitcast<u32>((_e3523 & 31))) & bitcast<u32>(((1 << bitcast<u32>(_e3527)) - 1))));
                    } else {
                        let _e3536 = (_e3523 & 31);
                        let _e3537 = (32 - _e3536);
                        let _e3539 = local_4[_e3534];
                        let _e3551 = local_4[(_e3534 + 1)];
                        phi_42543_ = (bitcast<i32>(((_e3539 >> bitcast<u32>(_e3536)) & bitcast<u32>(((1 << bitcast<u32>(_e3537)) - 1)))) | (bitcast<i32>(((_e3551 >> bitcast<u32>(0)) & bitcast<u32>(((1 << bitcast<u32>((_e3527 - _e3537))) - 1)))) << bitcast<u32>(_e3537)));
                    }
                    let _e3575 = phi_42543_;
                    phi_42542_ = _e3575;
                }
                let _e3577 = phi_42542_;
                local_5 = array<i32,8u>(0, 9, 18, 27, 37, 46, 55, 64);
                let _e3579 = local_5[_e3577];
                phi_42794_ = vec3<i32>(select(_e3460, 65535, _e3469.x), select(_e3464, 65535, _e3469.y), select(_e3468, 65535, _e3469.z));
                phi_42791_ = vec3<i32>(select(_e3501, 65535, _e3510.x), select(_e3505, 65535, _e3510.y), select(_e3509, 65535, _e3510.z));
                phi_42788_ = _e3579;
                break;
            }
            case 3: {
                let _e3150 = bitcast<vec3<u32>>(vec3<i32>(bitcast<i32>(((_e186.x >> bitcast<u32>(5)) & 1023u)), bitcast<i32>(((_e186.x >> bitcast<u32>(15)) & 1023u)), (bitcast<i32>(((_e186.x >> bitcast<u32>(25)) & 127u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(0)) & 7u)) << bitcast<u32>(7)))));
                let _e3164 = bitcast<vec3<i32>>(vec3<u32>(((_e3150.x >> bitcast<u32>(0)) & 1023u), ((_e3150.y >> bitcast<u32>(0)) & 1023u), ((_e3150.z >> bitcast<u32>(0)) & 1023u)));
                let _e3169 = (((_e3164 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(9, 9, 9)));
                let _e3170 = (_e3164 == vec3<i32>(0, 0, 0));
                if _e3170.x {
                    phi_42413_ = 0;
                } else {
                    phi_42413_ = _e3169.x;
                }
                let _e3174 = phi_42413_;
                if _e3170.y {
                    phi_42414_ = 0;
                } else {
                    phi_42414_ = _e3169.y;
                }
                let _e3178 = phi_42414_;
                if _e3170.z {
                    phi_42415_ = 0;
                } else {
                    phi_42415_ = _e3169.z;
                }
                let _e3182 = phi_42415_;
                let _e3183 = (_e3164 == vec3<i32>(1023, 1023, 1023));
                let _e3191 = bitcast<vec3<u32>>(vec3<i32>(bitcast<i32>(((_e186.y >> bitcast<u32>(3)) & 1023u)), bitcast<i32>(((_e186.y >> bitcast<u32>(13)) & 1023u)), (bitcast<i32>(((_e186.y >> bitcast<u32>(23)) & 511u)) | (bitcast<i32>(((_e186.z >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(9)))));
                let _e3205 = bitcast<vec3<i32>>(vec3<u32>(((_e3191.x >> bitcast<u32>(0)) & 1023u), ((_e3191.y >> bitcast<u32>(0)) & 1023u), ((_e3191.z >> bitcast<u32>(0)) & 1023u)));
                let _e3210 = (((_e3205 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(9, 9, 9)));
                let _e3211 = (_e3205 == vec3<i32>(0, 0, 0));
                if _e3211.x {
                    phi_42430_ = 0;
                } else {
                    phi_42430_ = _e3210.x;
                }
                let _e3215 = phi_42430_;
                if _e3211.y {
                    phi_42431_ = 0;
                } else {
                    phi_42431_ = _e3210.y;
                }
                let _e3219 = phi_42431_;
                if _e3211.z {
                    phi_42432_ = 0;
                } else {
                    phi_42432_ = _e3210.z;
                }
                let _e3223 = phi_42432_;
                let _e3224 = (_e3205 == vec3<i32>(1023, 1023, 1023));
                let _e3234 = max((64 + (_e185 * 4)), 65);
                let _e3236 = select(4, 3, (_e185 == 0));
                local_6 = _e186;
                if (_e3236 <= 0) {
                    phi_42440_ = 0;
                } else {
                    let _e3243 = (_e3234 >> bitcast<u32>(5));
                    if ((((_e3234 + _e3236) - 1) >> bitcast<u32>(5)) == _e3243) {
                        let _e3274 = local_6[_e3243];
                        phi_42441_ = bitcast<i32>(((_e3274 >> bitcast<u32>((_e3234 & 31))) & bitcast<u32>(((1 << bitcast<u32>(_e3236)) - 1))));
                    } else {
                        let _e3245 = (_e3234 & 31);
                        let _e3246 = (32 - _e3245);
                        let _e3248 = local_6[_e3243];
                        let _e3260 = local_6[(_e3243 + 1)];
                        phi_42441_ = (bitcast<i32>(((_e3248 >> bitcast<u32>(_e3245)) & bitcast<u32>(((1 << bitcast<u32>(_e3246)) - 1)))) | (bitcast<i32>(((_e3260 >> bitcast<u32>(0)) & bitcast<u32>(((1 << bitcast<u32>((_e3236 - _e3246))) - 1)))) << bitcast<u32>(_e3246)));
                    }
                    let _e3284 = phi_42441_;
                    phi_42440_ = _e3284;
                }
                let _e3286 = phi_42440_;
                local_7 = array<i32,16u>(0, 4, 9, 13, 17, 21, 26, 30, 34, 38, 43, 47, 51, 55, 60, 64);
                let _e3288 = local_7[_e3286];
                phi_42794_ = vec3<i32>(select(_e3174, 65535, _e3183.x), select(_e3178, 65535, _e3183.y), select(_e3182, 65535, _e3183.z));
                phi_42791_ = vec3<i32>(select(_e3215, 65535, _e3224.x), select(_e3219, 65535, _e3224.y), select(_e3223, 65535, _e3224.z));
                phi_42788_ = _e3288;
                break;
            }
            case 6: {
                let _e2845 = vec3<i32>((bitcast<i32>(((_e186.x >> bitcast<u32>(5)) & 1023u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(7)) & 1u)) << bitcast<u32>(10))), (bitcast<i32>(((_e186.x >> bitcast<u32>(15)) & 1023u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(18)) & 1u)) << bitcast<u32>(10))), ((bitcast<i32>(((_e186.x >> bitcast<u32>(25)) & 127u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(0)) & 7u)) << bitcast<u32>(7))) | (bitcast<i32>(((_e186.y >> bitcast<u32>(27)) & 1u)) << bitcast<u32>(10))));
                if (_e201 != 0) {
                    let _e2871 = bitcast<i32>(_e186.z);
                    phi_42371_ = (vec3<i32>(((((_e2871 >> bitcast<u32>(7)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)), (bitcast<i32>(((_e186.y >> bitcast<u32>(19)) & 15u)) | (((((bitcast<i32>(_e186.y) >> bitcast<u32>(8)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4))), (((bitcast<i32>(((_e186.z >> bitcast<u32>(5)) & 1u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(28)) & 1u)) << bitcast<u32>(1))) | (bitcast<i32>(((_e186.z >> bitcast<u32>(6)) & 1u)) << bitcast<u32>(2))) | (((((_e2871 >> bitcast<u32>(12)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(3)))) + _e2845);
                    phi_42359_ = (_e2845 + vec3<i32>(((((_e2871 >> bitcast<u32>(1)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)), (bitcast<i32>(((_e186.y >> bitcast<u32>(9)) & 15u)) | (((((_e2871 >> bitcast<u32>(11)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4))), (bitcast<i32>(((_e186.y >> bitcast<u32>(29)) & 7u)) | (((((_e2871 >> bitcast<u32>(0)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(3)))));
                } else {
                    let _e2847 = bitcast<i32>(_e186.y);
                    phi_42371_ = (vec3<i32>(((((_e2847 >> bitcast<u32>(3)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)), ((((_e2847 >> bitcast<u32>(13)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), ((((_e2847 >> bitcast<u32>(23)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u))) + _e2845);
                    phi_42359_ = _e2845;
                }
                let _e2962 = phi_42371_;
                let _e2964 = phi_42359_;
                let _e2965 = bitcast<vec3<u32>>(_e2964);
                let _e2979 = bitcast<vec3<i32>>(vec3<u32>(((_e2965.x >> bitcast<u32>(0)) & 2047u), ((_e2965.y >> bitcast<u32>(0)) & 2047u), ((_e2965.z >> bitcast<u32>(0)) & 2047u)));
                let _e2984 = (((_e2979 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(10, 10, 10)));
                let _e2985 = (_e2979 == vec3<i32>(0, 0, 0));
                if _e2985.x {
                    phi_42360_ = 0;
                } else {
                    phi_42360_ = _e2984.x;
                }
                let _e2989 = phi_42360_;
                if _e2985.y {
                    phi_42361_ = 0;
                } else {
                    phi_42361_ = _e2984.y;
                }
                let _e2993 = phi_42361_;
                if _e2985.z {
                    phi_42362_ = 0;
                } else {
                    phi_42362_ = _e2984.z;
                }
                let _e2997 = phi_42362_;
                let _e2998 = (_e2979 == vec3<i32>(2047, 2047, 2047));
                let _e3006 = bitcast<vec3<u32>>(_e2962);
                let _e3020 = bitcast<vec3<i32>>(vec3<u32>(((_e3006.x >> bitcast<u32>(0)) & 2047u), ((_e3006.y >> bitcast<u32>(0)) & 2047u), ((_e3006.z >> bitcast<u32>(0)) & 2047u)));
                let _e3025 = (((_e3020 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(10, 10, 10)));
                let _e3026 = (_e3020 == vec3<i32>(0, 0, 0));
                if _e3026.x {
                    phi_42378_ = 0;
                } else {
                    phi_42378_ = _e3025.x;
                }
                let _e3030 = phi_42378_;
                if _e3026.y {
                    phi_42379_ = 0;
                } else {
                    phi_42379_ = _e3025.y;
                }
                let _e3034 = phi_42379_;
                if _e3026.z {
                    phi_42380_ = 0;
                } else {
                    phi_42380_ = _e3025.z;
                }
                let _e3038 = phi_42380_;
                let _e3039 = (_e3020 == vec3<i32>(2047, 2047, 2047));
                let _e3052 = max(((81 + (_e185 * 3)) - select(0, 1, (_e185 > _e203))), 82);
                let _e3056 = select(3, 2, ((_e185 == 0) || (_e185 == _e203)));
                local_8 = _e186;
                if (_e3056 <= 0) {
                    phi_42388_ = 0;
                } else {
                    let _e3063 = (_e3052 >> bitcast<u32>(5));
                    if ((((_e3052 + _e3056) - 1) >> bitcast<u32>(5)) == _e3063) {
                        let _e3094 = local_8[_e3063];
                        phi_42389_ = bitcast<i32>(((_e3094 >> bitcast<u32>((_e3052 & 31))) & bitcast<u32>(((1 << bitcast<u32>(_e3056)) - 1))));
                    } else {
                        let _e3065 = (_e3052 & 31);
                        let _e3066 = (32 - _e3065);
                        let _e3068 = local_8[_e3063];
                        let _e3080 = local_8[(_e3063 + 1)];
                        phi_42389_ = (bitcast<i32>(((_e3068 >> bitcast<u32>(_e3065)) & bitcast<u32>(((1 << bitcast<u32>(_e3066)) - 1)))) | (bitcast<i32>(((_e3080 >> bitcast<u32>(0)) & bitcast<u32>(((1 << bitcast<u32>((_e3056 - _e3066))) - 1)))) << bitcast<u32>(_e3066)));
                    }
                    let _e3104 = phi_42389_;
                    phi_42388_ = _e3104;
                }
                let _e3106 = phi_42388_;
                local_9 = array<i32,8u>(0, 9, 18, 27, 37, 46, 55, 64);
                let _e3108 = local_9[_e3106];
                phi_42794_ = vec3<i32>(select(_e2989, 65535, _e2998.x), select(_e2993, 65535, _e2998.y), select(_e2997, 65535, _e2998.z));
                phi_42791_ = vec3<i32>(select(_e3030, 65535, _e3039.x), select(_e3034, 65535, _e3039.y), select(_e3038, 65535, _e3039.z));
                phi_42788_ = _e3108;
                break;
            }
            case 7: {
                let _e2608 = (bitcast<i32>(((_e186.x >> bitcast<u32>(5)) & 1023u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(12)) & 1u)) << bitcast<u32>(10)));
                let _e2619 = (bitcast<i32>(((_e186.x >> bitcast<u32>(15)) & 1023u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(22)) & 1u)) << bitcast<u32>(10)));
                let _e2637 = ((bitcast<i32>(((_e186.x >> bitcast<u32>(25)) & 127u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(0)) & 7u)) << bitcast<u32>(7))) | (bitcast<i32>(((_e186.z >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(10)));
                let _e2638 = bitcast<i32>(_e186.y);
                let _e2665 = bitcast<vec3<u32>>(vec3<i32>(_e2608, _e2619, _e2637));
                let _e2679 = bitcast<vec3<i32>>(vec3<u32>(((_e2665.x >> bitcast<u32>(0)) & 2047u), ((_e2665.y >> bitcast<u32>(0)) & 2047u), ((_e2665.z >> bitcast<u32>(0)) & 2047u)));
                let _e2684 = (((_e2679 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(10, 10, 10)));
                let _e2685 = (_e2679 == vec3<i32>(0, 0, 0));
                if _e2685.x {
                    phi_42251_ = 0;
                } else {
                    phi_42251_ = _e2684.x;
                }
                let _e2689 = phi_42251_;
                if _e2685.y {
                    phi_42252_ = 0;
                } else {
                    phi_42252_ = _e2684.y;
                }
                let _e2693 = phi_42252_;
                if _e2685.z {
                    phi_42253_ = 0;
                } else {
                    phi_42253_ = _e2684.z;
                }
                let _e2697 = phi_42253_;
                let _e2698 = (_e2679 == vec3<i32>(2047, 2047, 2047));
                let _e2706 = bitcast<vec3<u32>>(vec3<i32>((((((_e2638 >> bitcast<u32>(3)) & 511) << bitcast<u32>(23u)) >> bitcast<u32>(23u)) + _e2608), (((((_e2638 >> bitcast<u32>(13)) & 511) << bitcast<u32>(23u)) >> bitcast<u32>(23u)) + _e2619), (((((_e2638 >> bitcast<u32>(23)) & 511) << bitcast<u32>(23u)) >> bitcast<u32>(23u)) + _e2637)));
                let _e2720 = bitcast<vec3<i32>>(vec3<u32>(((_e2706.x >> bitcast<u32>(0)) & 2047u), ((_e2706.y >> bitcast<u32>(0)) & 2047u), ((_e2706.z >> bitcast<u32>(0)) & 2047u)));
                let _e2725 = (((_e2720 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(10, 10, 10)));
                let _e2726 = (_e2720 == vec3<i32>(0, 0, 0));
                if _e2726.x {
                    phi_42268_ = 0;
                } else {
                    phi_42268_ = _e2725.x;
                }
                let _e2730 = phi_42268_;
                if _e2726.y {
                    phi_42269_ = 0;
                } else {
                    phi_42269_ = _e2725.y;
                }
                let _e2734 = phi_42269_;
                if _e2726.z {
                    phi_42270_ = 0;
                } else {
                    phi_42270_ = _e2725.z;
                }
                let _e2738 = phi_42270_;
                let _e2739 = (_e2720 == vec3<i32>(2047, 2047, 2047));
                let _e2749 = max((64 + (_e185 * 4)), 65);
                let _e2751 = select(4, 3, (_e185 == 0));
                local_10 = _e186;
                if (_e2751 <= 0) {
                    phi_42278_ = 0;
                } else {
                    let _e2758 = (_e2749 >> bitcast<u32>(5));
                    if ((((_e2749 + _e2751) - 1) >> bitcast<u32>(5)) == _e2758) {
                        let _e2789 = local_10[_e2758];
                        phi_42279_ = bitcast<i32>(((_e2789 >> bitcast<u32>((_e2749 & 31))) & bitcast<u32>(((1 << bitcast<u32>(_e2751)) - 1))));
                    } else {
                        let _e2760 = (_e2749 & 31);
                        let _e2761 = (32 - _e2760);
                        let _e2763 = local_10[_e2758];
                        let _e2775 = local_10[(_e2758 + 1)];
                        phi_42279_ = (bitcast<i32>(((_e2763 >> bitcast<u32>(_e2760)) & bitcast<u32>(((1 << bitcast<u32>(_e2761)) - 1)))) | (bitcast<i32>(((_e2775 >> bitcast<u32>(0)) & bitcast<u32>(((1 << bitcast<u32>((_e2751 - _e2761))) - 1)))) << bitcast<u32>(_e2761)));
                    }
                    let _e2799 = phi_42279_;
                    phi_42278_ = _e2799;
                }
                let _e2801 = phi_42278_;
                local_11 = array<i32,16u>(0, 4, 9, 13, 17, 21, 26, 30, 34, 38, 43, 47, 51, 55, 60, 64);
                let _e2803 = local_11[_e2801];
                phi_42794_ = vec3<i32>(select(_e2689, 65535, _e2698.x), select(_e2693, 65535, _e2698.y), select(_e2697, 65535, _e2698.z));
                phi_42791_ = vec3<i32>(select(_e2730, 65535, _e2739.x), select(_e2734, 65535, _e2739.y), select(_e2738, 65535, _e2739.z));
                phi_42788_ = _e2803;
                break;
            }
            case 10: {
                let _e2340 = vec3<i32>((bitcast<i32>(((_e186.x >> bitcast<u32>(5)) & 1023u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(7)) & 1u)) << bitcast<u32>(10))), (bitcast<i32>(((_e186.x >> bitcast<u32>(15)) & 1023u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(17)) & 1u)) << bitcast<u32>(10))), ((bitcast<i32>(((_e186.x >> bitcast<u32>(25)) & 127u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(0)) & 7u)) << bitcast<u32>(7))) | (bitcast<i32>(((_e186.y >> bitcast<u32>(28)) & 1u)) << bitcast<u32>(10))));
                if (_e201 != 0) {
                    let _e2366 = bitcast<i32>(_e186.z);
                    let _e2374 = bitcast<i32>(_e186.y);
                    phi_42197_ = (vec3<i32>(((((_e2366 >> bitcast<u32>(7)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)), ((((_e2374 >> bitcast<u32>(19)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)), (((bitcast<i32>(((_e186.y >> bitcast<u32>(18)) & 1u)) | (bitcast<i32>(((_e186.z >> bitcast<u32>(5)) & 3u)) << bitcast<u32>(1))) | (bitcast<i32>(((_e186.z >> bitcast<u32>(12)) & 1u)) << bitcast<u32>(3))) | (((((_e2366 >> bitcast<u32>(11)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4)))) + _e2340);
                    phi_42185_ = (_e2340 + vec3<i32>(((((_e2366 >> bitcast<u32>(1)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)), ((((_e2374 >> bitcast<u32>(9)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)), ((bitcast<i32>(((_e186.y >> bitcast<u32>(29)) & 7u)) | (bitcast<i32>(((_e186.z >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(3))) | (((((_e2374 >> bitcast<u32>(8)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4)))));
                } else {
                    let _e2342 = bitcast<i32>(_e186.y);
                    phi_42197_ = (vec3<i32>(((((_e2342 >> bitcast<u32>(3)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)), ((((_e2342 >> bitcast<u32>(13)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)), ((((_e2342 >> bitcast<u32>(23)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u))) + _e2340);
                    phi_42185_ = _e2340;
                }
                let _e2450 = phi_42197_;
                let _e2452 = phi_42185_;
                let _e2453 = bitcast<vec3<u32>>(_e2452);
                let _e2467 = bitcast<vec3<i32>>(vec3<u32>(((_e2453.x >> bitcast<u32>(0)) & 2047u), ((_e2453.y >> bitcast<u32>(0)) & 2047u), ((_e2453.z >> bitcast<u32>(0)) & 2047u)));
                let _e2472 = (((_e2467 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(10, 10, 10)));
                let _e2473 = (_e2467 == vec3<i32>(0, 0, 0));
                if _e2473.x {
                    phi_42186_ = 0;
                } else {
                    phi_42186_ = _e2472.x;
                }
                let _e2477 = phi_42186_;
                if _e2473.y {
                    phi_42187_ = 0;
                } else {
                    phi_42187_ = _e2472.y;
                }
                let _e2481 = phi_42187_;
                if _e2473.z {
                    phi_42188_ = 0;
                } else {
                    phi_42188_ = _e2472.z;
                }
                let _e2485 = phi_42188_;
                let _e2486 = (_e2467 == vec3<i32>(2047, 2047, 2047));
                let _e2494 = bitcast<vec3<u32>>(_e2450);
                let _e2508 = bitcast<vec3<i32>>(vec3<u32>(((_e2494.x >> bitcast<u32>(0)) & 2047u), ((_e2494.y >> bitcast<u32>(0)) & 2047u), ((_e2494.z >> bitcast<u32>(0)) & 2047u)));
                let _e2513 = (((_e2508 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(10, 10, 10)));
                let _e2514 = (_e2508 == vec3<i32>(0, 0, 0));
                if _e2514.x {
                    phi_42204_ = 0;
                } else {
                    phi_42204_ = _e2513.x;
                }
                let _e2518 = phi_42204_;
                if _e2514.y {
                    phi_42205_ = 0;
                } else {
                    phi_42205_ = _e2513.y;
                }
                let _e2522 = phi_42205_;
                if _e2514.z {
                    phi_42206_ = 0;
                } else {
                    phi_42206_ = _e2513.z;
                }
                let _e2526 = phi_42206_;
                let _e2527 = (_e2508 == vec3<i32>(2047, 2047, 2047));
                let _e2540 = max(((81 + (_e185 * 3)) - select(0, 1, (_e185 > _e203))), 82);
                let _e2544 = select(3, 2, ((_e185 == 0) || (_e185 == _e203)));
                local_12 = _e186;
                if (_e2544 <= 0) {
                    phi_42214_ = 0;
                } else {
                    let _e2551 = (_e2540 >> bitcast<u32>(5));
                    if ((((_e2540 + _e2544) - 1) >> bitcast<u32>(5)) == _e2551) {
                        let _e2582 = local_12[_e2551];
                        phi_42215_ = bitcast<i32>(((_e2582 >> bitcast<u32>((_e2540 & 31))) & bitcast<u32>(((1 << bitcast<u32>(_e2544)) - 1))));
                    } else {
                        let _e2553 = (_e2540 & 31);
                        let _e2554 = (32 - _e2553);
                        let _e2556 = local_12[_e2551];
                        let _e2568 = local_12[(_e2551 + 1)];
                        phi_42215_ = (bitcast<i32>(((_e2556 >> bitcast<u32>(_e2553)) & bitcast<u32>(((1 << bitcast<u32>(_e2554)) - 1)))) | (bitcast<i32>(((_e2568 >> bitcast<u32>(0)) & bitcast<u32>(((1 << bitcast<u32>((_e2544 - _e2554))) - 1)))) << bitcast<u32>(_e2554)));
                    }
                    let _e2592 = phi_42215_;
                    phi_42214_ = _e2592;
                }
                let _e2594 = phi_42214_;
                local_13 = array<i32,8u>(0, 9, 18, 27, 37, 46, 55, 64);
                let _e2596 = local_13[_e2594];
                phi_42794_ = vec3<i32>(select(_e2477, 65535, _e2486.x), select(_e2481, 65535, _e2486.y), select(_e2485, 65535, _e2486.z));
                phi_42791_ = vec3<i32>(select(_e2518, 65535, _e2527.x), select(_e2522, 65535, _e2527.y), select(_e2526, 65535, _e2527.z));
                phi_42788_ = _e2596;
                break;
            }
            case 11: {
                let _e1982 = ((_e186.y >> bitcast<u32>(11)) & 3u);
                let _e1989 = (((_e1982 >> bitcast<u32>(1)) & 1431655765u) | ((_e1982 & 1431655765u) << bitcast<u32>(1)));
                let _e1996 = (((_e1989 >> bitcast<u32>(2)) & 858993459u) | ((_e1989 & 858993459u) << bitcast<u32>(2)));
                let _e2003 = (((_e1996 >> bitcast<u32>(4)) & 252645135u) | ((_e1996 & 252645135u) << bitcast<u32>(4)));
                let _e2010 = (((_e2003 >> bitcast<u32>(8)) & 16711935u) | ((_e2003 & 16711935u) << bitcast<u32>(8)));
                let _e2023 = (bitcast<i32>(((_e186.x >> bitcast<u32>(5)) & 1023u)) | (bitcast<i32>(((((_e2010 >> bitcast<u32>(16)) & 65535u) | ((_e2010 & 65535u) << bitcast<u32>(16))) >> bitcast<u32>(30))) << bitcast<u32>(10)));
                let _e2030 = ((_e186.y >> bitcast<u32>(21)) & 3u);
                let _e2037 = (((_e2030 >> bitcast<u32>(1)) & 1431655765u) | ((_e2030 & 1431655765u) << bitcast<u32>(1)));
                let _e2044 = (((_e2037 >> bitcast<u32>(2)) & 858993459u) | ((_e2037 & 858993459u) << bitcast<u32>(2)));
                let _e2051 = (((_e2044 >> bitcast<u32>(4)) & 252645135u) | ((_e2044 & 252645135u) << bitcast<u32>(4)));
                let _e2058 = (((_e2051 >> bitcast<u32>(8)) & 16711935u) | ((_e2051 & 16711935u) << bitcast<u32>(8)));
                let _e2071 = (bitcast<i32>(((_e186.x >> bitcast<u32>(15)) & 1023u)) | (bitcast<i32>(((((_e2058 >> bitcast<u32>(16)) & 65535u) | ((_e2058 & 65535u) << bitcast<u32>(16))) >> bitcast<u32>(30))) << bitcast<u32>(10)));
                let _e2091 = (((_e186.y >> bitcast<u32>(31)) & 1u) | (((_e186.z >> bitcast<u32>(0)) & 1u) << bitcast<u32>(1)));
                let _e2098 = (((_e2091 >> bitcast<u32>(1)) & 1431655765u) | ((_e2091 & 1431655765u) << bitcast<u32>(1)));
                let _e2105 = (((_e2098 >> bitcast<u32>(2)) & 858993459u) | ((_e2098 & 858993459u) << bitcast<u32>(2)));
                let _e2112 = (((_e2105 >> bitcast<u32>(4)) & 252645135u) | ((_e2105 & 252645135u) << bitcast<u32>(4)));
                let _e2119 = (((_e2112 >> bitcast<u32>(8)) & 16711935u) | ((_e2112 & 16711935u) << bitcast<u32>(8)));
                let _e2132 = ((bitcast<i32>(((_e186.x >> bitcast<u32>(25)) & 127u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(0)) & 7u)) << bitcast<u32>(7))) | (bitcast<i32>(((((_e2119 >> bitcast<u32>(16)) & 65535u) | ((_e2119 & 65535u) << bitcast<u32>(16))) >> bitcast<u32>(30))) << bitcast<u32>(10)));
                let _e2133 = bitcast<i32>(_e186.y);
                let _e2160 = bitcast<vec3<u32>>(vec3<i32>(_e2023, _e2071, _e2132));
                let _e2174 = bitcast<vec3<i32>>(vec3<u32>(((_e2160.x >> bitcast<u32>(0)) & 4095u), ((_e2160.y >> bitcast<u32>(0)) & 4095u), ((_e2160.z >> bitcast<u32>(0)) & 4095u)));
                let _e2179 = (((_e2174 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(11, 11, 11)));
                let _e2180 = (_e2174 == vec3<i32>(0, 0, 0));
                if _e2180.x {
                    phi_42081_ = 0;
                } else {
                    phi_42081_ = _e2179.x;
                }
                let _e2184 = phi_42081_;
                if _e2180.y {
                    phi_42082_ = 0;
                } else {
                    phi_42082_ = _e2179.y;
                }
                let _e2188 = phi_42082_;
                if _e2180.z {
                    phi_42083_ = 0;
                } else {
                    phi_42083_ = _e2179.z;
                }
                let _e2192 = phi_42083_;
                let _e2193 = (_e2174 == vec3<i32>(4095, 4095, 4095));
                let _e2201 = bitcast<vec3<u32>>(vec3<i32>((((((_e2133 >> bitcast<u32>(3)) & 255) << bitcast<u32>(24u)) >> bitcast<u32>(24u)) + _e2023), (((((_e2133 >> bitcast<u32>(13)) & 255) << bitcast<u32>(24u)) >> bitcast<u32>(24u)) + _e2071), (((((_e2133 >> bitcast<u32>(23)) & 255) << bitcast<u32>(24u)) >> bitcast<u32>(24u)) + _e2132)));
                let _e2215 = bitcast<vec3<i32>>(vec3<u32>(((_e2201.x >> bitcast<u32>(0)) & 4095u), ((_e2201.y >> bitcast<u32>(0)) & 4095u), ((_e2201.z >> bitcast<u32>(0)) & 4095u)));
                let _e2220 = (((_e2215 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(11, 11, 11)));
                let _e2221 = (_e2215 == vec3<i32>(0, 0, 0));
                if _e2221.x {
                    phi_42098_ = 0;
                } else {
                    phi_42098_ = _e2220.x;
                }
                let _e2225 = phi_42098_;
                if _e2221.y {
                    phi_42099_ = 0;
                } else {
                    phi_42099_ = _e2220.y;
                }
                let _e2229 = phi_42099_;
                if _e2221.z {
                    phi_42100_ = 0;
                } else {
                    phi_42100_ = _e2220.z;
                }
                let _e2233 = phi_42100_;
                let _e2234 = (_e2215 == vec3<i32>(4095, 4095, 4095));
                let _e2244 = max((64 + (_e185 * 4)), 65);
                let _e2246 = select(4, 3, (_e185 == 0));
                local_14 = _e186;
                if (_e2246 <= 0) {
                    phi_42108_ = 0;
                } else {
                    let _e2253 = (_e2244 >> bitcast<u32>(5));
                    if ((((_e2244 + _e2246) - 1) >> bitcast<u32>(5)) == _e2253) {
                        let _e2284 = local_14[_e2253];
                        phi_42109_ = bitcast<i32>(((_e2284 >> bitcast<u32>((_e2244 & 31))) & bitcast<u32>(((1 << bitcast<u32>(_e2246)) - 1))));
                    } else {
                        let _e2255 = (_e2244 & 31);
                        let _e2256 = (32 - _e2255);
                        let _e2258 = local_14[_e2253];
                        let _e2270 = local_14[(_e2253 + 1)];
                        phi_42109_ = (bitcast<i32>(((_e2258 >> bitcast<u32>(_e2255)) & bitcast<u32>(((1 << bitcast<u32>(_e2256)) - 1)))) | (bitcast<i32>(((_e2270 >> bitcast<u32>(0)) & bitcast<u32>(((1 << bitcast<u32>((_e2246 - _e2256))) - 1)))) << bitcast<u32>(_e2256)));
                    }
                    let _e2294 = phi_42109_;
                    phi_42108_ = _e2294;
                }
                let _e2296 = phi_42108_;
                local_15 = array<i32,16u>(0, 4, 9, 13, 17, 21, 26, 30, 34, 38, 43, 47, 51, 55, 60, 64);
                let _e2298 = local_15[_e2296];
                phi_42794_ = vec3<i32>(select(_e2184, 65535, _e2193.x), select(_e2188, 65535, _e2193.y), select(_e2192, 65535, _e2193.z));
                phi_42791_ = vec3<i32>(select(_e2225, 65535, _e2234.x), select(_e2229, 65535, _e2234.y), select(_e2233, 65535, _e2234.z));
                phi_42788_ = _e2298;
                break;
            }
            case 14: {
                let _e1696 = vec3<i32>(bitcast<i32>(((_e186.x >> bitcast<u32>(5)) & 511u)), bitcast<i32>(((_e186.x >> bitcast<u32>(15)) & 511u)), (bitcast<i32>(((_e186.x >> bitcast<u32>(25)) & 127u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(0)) & 3u)) << bitcast<u32>(7))));
                if (_e201 != 0) {
                    let _e1722 = bitcast<i32>(_e186.z);
                    let _e1734 = bitcast<i32>(_e186.x);
                    let _e1777 = bitcast<i32>(_e186.y);
                    phi_42027_ = (vec3<i32>(((((_e1722 >> bitcast<u32>(7)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), (bitcast<i32>(((_e186.y >> bitcast<u32>(19)) & 15u)) | (((((_e1777 >> bitcast<u32>(8)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4))), ((((bitcast<i32>(((_e186.y >> bitcast<u32>(18)) & 1u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(28)) & 1u)) << bitcast<u32>(1))) | (bitcast<i32>(((_e186.z >> bitcast<u32>(6)) & 1u)) << bitcast<u32>(2))) | (bitcast<i32>(((_e186.z >> bitcast<u32>(12)) & 1u)) << bitcast<u32>(3))) | (((((_e1777 >> bitcast<u32>(2)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4)))) + _e1696);
                    phi_42015_ = (_e1696 + vec3<i32>(((((_e1722 >> bitcast<u32>(1)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), (bitcast<i32>(((_e186.y >> bitcast<u32>(9)) & 15u)) | (((((_e1734 >> bitcast<u32>(24)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4))), ((bitcast<i32>(((_e186.y >> bitcast<u32>(29)) & 7u)) | (bitcast<i32>(((_e186.z >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(3))) | (((((_e1734 >> bitcast<u32>(14)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4)))));
                } else {
                    let _e1698 = bitcast<i32>(_e186.y);
                    phi_42027_ = (vec3<i32>(((((_e1698 >> bitcast<u32>(3)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), ((((_e1698 >> bitcast<u32>(13)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), ((((_e1698 >> bitcast<u32>(23)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u))) + _e1696);
                    phi_42015_ = _e1696;
                }
                let _e1828 = phi_42027_;
                let _e1830 = phi_42015_;
                let _e1831 = bitcast<vec3<u32>>(_e1830);
                let _e1845 = bitcast<vec3<i32>>(vec3<u32>(((_e1831.x >> bitcast<u32>(0)) & 511u), ((_e1831.y >> bitcast<u32>(0)) & 511u), ((_e1831.z >> bitcast<u32>(0)) & 511u)));
                let _e1850 = (((_e1845 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(8, 8, 8)));
                let _e1851 = (_e1845 == vec3<i32>(0, 0, 0));
                if _e1851.x {
                    phi_42016_ = 0;
                } else {
                    phi_42016_ = _e1850.x;
                }
                let _e1855 = phi_42016_;
                if _e1851.y {
                    phi_42017_ = 0;
                } else {
                    phi_42017_ = _e1850.y;
                }
                let _e1859 = phi_42017_;
                if _e1851.z {
                    phi_42018_ = 0;
                } else {
                    phi_42018_ = _e1850.z;
                }
                let _e1863 = phi_42018_;
                let _e1864 = (_e1845 == vec3<i32>(511, 511, 511));
                let _e1872 = bitcast<vec3<u32>>(_e1828);
                let _e1886 = bitcast<vec3<i32>>(vec3<u32>(((_e1872.x >> bitcast<u32>(0)) & 511u), ((_e1872.y >> bitcast<u32>(0)) & 511u), ((_e1872.z >> bitcast<u32>(0)) & 511u)));
                let _e1891 = (((_e1886 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(8, 8, 8)));
                let _e1892 = (_e1886 == vec3<i32>(0, 0, 0));
                if _e1892.x {
                    phi_42034_ = 0;
                } else {
                    phi_42034_ = _e1891.x;
                }
                let _e1896 = phi_42034_;
                if _e1892.y {
                    phi_42035_ = 0;
                } else {
                    phi_42035_ = _e1891.y;
                }
                let _e1900 = phi_42035_;
                if _e1892.z {
                    phi_42036_ = 0;
                } else {
                    phi_42036_ = _e1891.z;
                }
                let _e1904 = phi_42036_;
                let _e1905 = (_e1886 == vec3<i32>(511, 511, 511));
                let _e1918 = max(((81 + (_e185 * 3)) - select(0, 1, (_e185 > _e203))), 82);
                let _e1922 = select(3, 2, ((_e185 == 0) || (_e185 == _e203)));
                local_16 = _e186;
                if (_e1922 <= 0) {
                    phi_42044_ = 0;
                } else {
                    let _e1929 = (_e1918 >> bitcast<u32>(5));
                    if ((((_e1918 + _e1922) - 1) >> bitcast<u32>(5)) == _e1929) {
                        let _e1960 = local_16[_e1929];
                        phi_42045_ = bitcast<i32>(((_e1960 >> bitcast<u32>((_e1918 & 31))) & bitcast<u32>(((1 << bitcast<u32>(_e1922)) - 1))));
                    } else {
                        let _e1931 = (_e1918 & 31);
                        let _e1932 = (32 - _e1931);
                        let _e1934 = local_16[_e1929];
                        let _e1946 = local_16[(_e1929 + 1)];
                        phi_42045_ = (bitcast<i32>(((_e1934 >> bitcast<u32>(_e1931)) & bitcast<u32>(((1 << bitcast<u32>(_e1932)) - 1)))) | (bitcast<i32>(((_e1946 >> bitcast<u32>(0)) & bitcast<u32>(((1 << bitcast<u32>((_e1922 - _e1932))) - 1)))) << bitcast<u32>(_e1932)));
                    }
                    let _e1970 = phi_42045_;
                    phi_42044_ = _e1970;
                }
                let _e1972 = phi_42044_;
                local_17 = array<i32,8u>(0, 9, 18, 27, 37, 46, 55, 64);
                let _e1974 = local_17[_e1972];
                phi_42794_ = vec3<i32>(select(_e1855, 65535, _e1864.x), select(_e1859, 65535, _e1864.y), select(_e1863, 65535, _e1864.z));
                phi_42791_ = vec3<i32>(select(_e1896, 65535, _e1905.x), select(_e1900, 65535, _e1905.y), select(_e1904, 65535, _e1905.z));
                phi_42788_ = _e1974;
                break;
            }
            case 15: {
                let _e1411 = ((_e186.y >> bitcast<u32>(7)) & 63u);
                let _e1418 = (((_e1411 >> bitcast<u32>(1)) & 1431655765u) | ((_e1411 & 1431655765u) << bitcast<u32>(1)));
                let _e1425 = (((_e1418 >> bitcast<u32>(2)) & 858993459u) | ((_e1418 & 858993459u) << bitcast<u32>(2)));
                let _e1432 = (((_e1425 >> bitcast<u32>(4)) & 252645135u) | ((_e1425 & 252645135u) << bitcast<u32>(4)));
                let _e1439 = (((_e1432 >> bitcast<u32>(8)) & 16711935u) | ((_e1432 & 16711935u) << bitcast<u32>(8)));
                let _e1452 = (bitcast<i32>(((_e186.x >> bitcast<u32>(5)) & 1023u)) | (bitcast<i32>(((((_e1439 >> bitcast<u32>(16)) & 65535u) | ((_e1439 & 65535u) << bitcast<u32>(16))) >> bitcast<u32>(26))) << bitcast<u32>(10)));
                let _e1459 = ((_e186.y >> bitcast<u32>(17)) & 63u);
                let _e1466 = (((_e1459 >> bitcast<u32>(1)) & 1431655765u) | ((_e1459 & 1431655765u) << bitcast<u32>(1)));
                let _e1473 = (((_e1466 >> bitcast<u32>(2)) & 858993459u) | ((_e1466 & 858993459u) << bitcast<u32>(2)));
                let _e1480 = (((_e1473 >> bitcast<u32>(4)) & 252645135u) | ((_e1473 & 252645135u) << bitcast<u32>(4)));
                let _e1487 = (((_e1480 >> bitcast<u32>(8)) & 16711935u) | ((_e1480 & 16711935u) << bitcast<u32>(8)));
                let _e1500 = (bitcast<i32>(((_e186.x >> bitcast<u32>(15)) & 1023u)) | (bitcast<i32>(((((_e1487 >> bitcast<u32>(16)) & 65535u) | ((_e1487 & 65535u) << bitcast<u32>(16))) >> bitcast<u32>(26))) << bitcast<u32>(10)));
                let _e1520 = (((_e186.y >> bitcast<u32>(27)) & 31u) | (((_e186.z >> bitcast<u32>(0)) & 1u) << bitcast<u32>(5)));
                let _e1527 = (((_e1520 >> bitcast<u32>(1)) & 1431655765u) | ((_e1520 & 1431655765u) << bitcast<u32>(1)));
                let _e1534 = (((_e1527 >> bitcast<u32>(2)) & 858993459u) | ((_e1527 & 858993459u) << bitcast<u32>(2)));
                let _e1541 = (((_e1534 >> bitcast<u32>(4)) & 252645135u) | ((_e1534 & 252645135u) << bitcast<u32>(4)));
                let _e1548 = (((_e1541 >> bitcast<u32>(8)) & 16711935u) | ((_e1541 & 16711935u) << bitcast<u32>(8)));
                let _e1561 = ((bitcast<i32>(((_e186.x >> bitcast<u32>(25)) & 127u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(0)) & 7u)) << bitcast<u32>(7))) | (bitcast<i32>(((((_e1548 >> bitcast<u32>(16)) & 65535u) | ((_e1548 & 65535u) << bitcast<u32>(16))) >> bitcast<u32>(26))) << bitcast<u32>(10)));
                let _e1562 = bitcast<i32>(_e186.y);
                let _e1589 = bitcast<vec3<u32>>(vec3<i32>(_e1452, _e1500, _e1561));
                let _e1604 = bitcast<vec3<u32>>(vec3<i32>((((((_e1562 >> bitcast<u32>(3)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)) + _e1452), (((((_e1562 >> bitcast<u32>(13)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)) + _e1500), (((((_e1562 >> bitcast<u32>(23)) & 15) << bitcast<u32>(28u)) >> bitcast<u32>(28u)) + _e1561)));
                let _e1621 = max((64 + (_e185 * 4)), 65);
                let _e1623 = select(4, 3, (_e185 == 0));
                local_18 = _e186;
                if (_e1623 <= 0) {
                    phi_41932_ = 0;
                } else {
                    let _e1630 = (_e1621 >> bitcast<u32>(5));
                    if ((((_e1621 + _e1623) - 1) >> bitcast<u32>(5)) == _e1630) {
                        let _e1661 = local_18[_e1630];
                        phi_41933_ = bitcast<i32>(((_e1661 >> bitcast<u32>((_e1621 & 31))) & bitcast<u32>(((1 << bitcast<u32>(_e1623)) - 1))));
                    } else {
                        let _e1632 = (_e1621 & 31);
                        let _e1633 = (32 - _e1632);
                        let _e1635 = local_18[_e1630];
                        let _e1647 = local_18[(_e1630 + 1)];
                        phi_41933_ = (bitcast<i32>(((_e1635 >> bitcast<u32>(_e1632)) & bitcast<u32>(((1 << bitcast<u32>(_e1633)) - 1)))) | (bitcast<i32>(((_e1647 >> bitcast<u32>(0)) & bitcast<u32>(((1 << bitcast<u32>((_e1623 - _e1633))) - 1)))) << bitcast<u32>(_e1633)));
                    }
                    let _e1671 = phi_41933_;
                    phi_41932_ = _e1671;
                }
                let _e1673 = phi_41932_;
                local_19 = array<i32,16u>(0, 4, 9, 13, 17, 21, 26, 30, 34, 38, 43, 47, 51, 55, 60, 64);
                let _e1675 = local_19[_e1673];
                phi_42794_ = bitcast<vec3<i32>>(vec3<u32>(((_e1589.x >> bitcast<u32>(0)) & 65535u), ((_e1589.y >> bitcast<u32>(0)) & 65535u), ((_e1589.z >> bitcast<u32>(0)) & 65535u)));
                phi_42791_ = bitcast<vec3<i32>>(vec3<u32>(((_e1604.x >> bitcast<u32>(0)) & 65535u), ((_e1604.y >> bitcast<u32>(0)) & 65535u), ((_e1604.z >> bitcast<u32>(0)) & 65535u)));
                phi_42788_ = _e1675;
                break;
            }
            case 18: {
                let _e1132 = vec3<i32>(bitcast<i32>(((_e186.x >> bitcast<u32>(5)) & 255u)), bitcast<i32>(((_e186.x >> bitcast<u32>(15)) & 255u)), (bitcast<i32>(((_e186.x >> bitcast<u32>(25)) & 127u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(7))));
                if (_e201 != 0) {
                    let _e1158 = bitcast<i32>(_e186.z);
                    let _e1170 = bitcast<i32>(_e186.x);
                    phi_41851_ = (vec3<i32>(((((_e1158 >> bitcast<u32>(7)) & 63) << bitcast<u32>(26u)) >> bitcast<u32>(26u)), (bitcast<i32>(((_e186.y >> bitcast<u32>(19)) & 15u)) | (((((_e1170 >> bitcast<u32>(13)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4))), (((bitcast<i32>(((_e186.y >> bitcast<u32>(18)) & 1u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(28)) & 1u)) << bitcast<u32>(1))) | (bitcast<i32>(((_e186.x >> bitcast<u32>(23)) & 1u)) << bitcast<u32>(2))) | (((((bitcast<i32>(_e186.y) >> bitcast<u32>(1)) & 3) << bitcast<u32>(30u)) >> bitcast<u32>(30u)) << bitcast<u32>(3)))) + _e1132);
                    phi_41839_ = (_e1132 + vec3<i32>(((((_e1158 >> bitcast<u32>(1)) & 63) << bitcast<u32>(26u)) >> bitcast<u32>(26u)), (bitcast<i32>(((_e186.y >> bitcast<u32>(9)) & 15u)) | (((((_e1170 >> bitcast<u32>(24)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4))), ((bitcast<i32>(((_e186.y >> bitcast<u32>(29)) & 7u)) | (bitcast<i32>(((_e186.z >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(3))) | (((((_e1170 >> bitcast<u32>(14)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4)))));
                } else {
                    let _e1134 = bitcast<i32>(_e186.y);
                    phi_41851_ = (vec3<i32>(((((_e1134 >> bitcast<u32>(3)) & 63) << bitcast<u32>(26u)) >> bitcast<u32>(26u)), ((((_e1134 >> bitcast<u32>(13)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), ((((_e1134 >> bitcast<u32>(23)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u))) + _e1132);
                    phi_41839_ = _e1132;
                }
                let _e1257 = phi_41851_;
                let _e1259 = phi_41839_;
                let _e1260 = bitcast<vec3<u32>>(_e1259);
                let _e1274 = bitcast<vec3<i32>>(vec3<u32>(((_e1260.x >> bitcast<u32>(0)) & 255u), ((_e1260.y >> bitcast<u32>(0)) & 255u), ((_e1260.z >> bitcast<u32>(0)) & 255u)));
                let _e1279 = (((_e1274 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(7, 7, 7)));
                let _e1280 = (_e1274 == vec3<i32>(0, 0, 0));
                if _e1280.x {
                    phi_41840_ = 0;
                } else {
                    phi_41840_ = _e1279.x;
                }
                let _e1284 = phi_41840_;
                if _e1280.y {
                    phi_41841_ = 0;
                } else {
                    phi_41841_ = _e1279.y;
                }
                let _e1288 = phi_41841_;
                if _e1280.z {
                    phi_41842_ = 0;
                } else {
                    phi_41842_ = _e1279.z;
                }
                let _e1292 = phi_41842_;
                let _e1293 = (_e1274 == vec3<i32>(255, 255, 255));
                let _e1301 = bitcast<vec3<u32>>(_e1257);
                let _e1315 = bitcast<vec3<i32>>(vec3<u32>(((_e1301.x >> bitcast<u32>(0)) & 255u), ((_e1301.y >> bitcast<u32>(0)) & 255u), ((_e1301.z >> bitcast<u32>(0)) & 255u)));
                let _e1320 = (((_e1315 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(7, 7, 7)));
                let _e1321 = (_e1315 == vec3<i32>(0, 0, 0));
                if _e1321.x {
                    phi_41858_ = 0;
                } else {
                    phi_41858_ = _e1320.x;
                }
                let _e1325 = phi_41858_;
                if _e1321.y {
                    phi_41859_ = 0;
                } else {
                    phi_41859_ = _e1320.y;
                }
                let _e1329 = phi_41859_;
                if _e1321.z {
                    phi_41860_ = 0;
                } else {
                    phi_41860_ = _e1320.z;
                }
                let _e1333 = phi_41860_;
                let _e1334 = (_e1315 == vec3<i32>(255, 255, 255));
                let _e1347 = max(((81 + (_e185 * 3)) - select(0, 1, (_e185 > _e203))), 82);
                let _e1351 = select(3, 2, ((_e185 == 0) || (_e185 == _e203)));
                local_20 = _e186;
                if (_e1351 <= 0) {
                    phi_41868_ = 0;
                } else {
                    let _e1358 = (_e1347 >> bitcast<u32>(5));
                    if ((((_e1347 + _e1351) - 1) >> bitcast<u32>(5)) == _e1358) {
                        let _e1389 = local_20[_e1358];
                        phi_41869_ = bitcast<i32>(((_e1389 >> bitcast<u32>((_e1347 & 31))) & bitcast<u32>(((1 << bitcast<u32>(_e1351)) - 1))));
                    } else {
                        let _e1360 = (_e1347 & 31);
                        let _e1361 = (32 - _e1360);
                        let _e1363 = local_20[_e1358];
                        let _e1375 = local_20[(_e1358 + 1)];
                        phi_41869_ = (bitcast<i32>(((_e1363 >> bitcast<u32>(_e1360)) & bitcast<u32>(((1 << bitcast<u32>(_e1361)) - 1)))) | (bitcast<i32>(((_e1375 >> bitcast<u32>(0)) & bitcast<u32>(((1 << bitcast<u32>((_e1351 - _e1361))) - 1)))) << bitcast<u32>(_e1361)));
                    }
                    let _e1399 = phi_41869_;
                    phi_41868_ = _e1399;
                }
                let _e1401 = phi_41868_;
                local_21 = array<i32,8u>(0, 9, 18, 27, 37, 46, 55, 64);
                let _e1403 = local_21[_e1401];
                phi_42794_ = vec3<i32>(select(_e1284, 65535, _e1293.x), select(_e1288, 65535, _e1293.y), select(_e1292, 65535, _e1293.z));
                phi_42791_ = vec3<i32>(select(_e1325, 65535, _e1334.x), select(_e1329, 65535, _e1334.y), select(_e1333, 65535, _e1334.z));
                phi_42788_ = _e1403;
                break;
            }
            case 22: {
                let _e819 = vec3<i32>(bitcast<i32>(((_e186.x >> bitcast<u32>(5)) & 255u)), bitcast<i32>(((_e186.x >> bitcast<u32>(15)) & 255u)), (bitcast<i32>(((_e186.x >> bitcast<u32>(25)) & 127u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(7))));
                if (_e201 != 0) {
                    let _e845 = bitcast<i32>(_e186.z);
                    let _e864 = bitcast<i32>(_e186.x);
                    let _e914 = bitcast<i32>(_e186.y);
                    phi_41743_ = (vec3<i32>(((((_e845 >> bitcast<u32>(7)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), ((bitcast<i32>(((_e186.y >> bitcast<u32>(19)) & 15u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(8)) & 1u)) << bitcast<u32>(4))) | (((((_e914 >> bitcast<u32>(1)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(5))), ((((bitcast<i32>(((_e186.x >> bitcast<u32>(13)) & 1u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(28)) & 1u)) << bitcast<u32>(1))) | (bitcast<i32>(((_e186.z >> bitcast<u32>(6)) & 1u)) << bitcast<u32>(2))) | (bitcast<i32>(((_e186.z >> bitcast<u32>(12)) & 1u)) << bitcast<u32>(3))) | (((((_e914 >> bitcast<u32>(2)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4)))) + _e819);
                    phi_41731_ = (_e819 + vec3<i32>(((((_e845 >> bitcast<u32>(1)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), ((bitcast<i32>(((_e186.y >> bitcast<u32>(9)) & 15u)) | (bitcast<i32>(((_e186.x >> bitcast<u32>(24)) & 1u)) << bitcast<u32>(4))) | (((((_e864 >> bitcast<u32>(23)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(5))), ((bitcast<i32>(((_e186.y >> bitcast<u32>(29)) & 7u)) | (bitcast<i32>(((_e186.z >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(3))) | (((((_e864 >> bitcast<u32>(14)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4)))));
                } else {
                    let _e821 = bitcast<i32>(_e186.y);
                    phi_41743_ = (vec3<i32>(((((_e821 >> bitcast<u32>(3)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), ((((_e821 >> bitcast<u32>(13)) & 63) << bitcast<u32>(26u)) >> bitcast<u32>(26u)), ((((_e821 >> bitcast<u32>(23)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u))) + _e819);
                    phi_41731_ = _e819;
                }
                let _e965 = phi_41743_;
                let _e967 = phi_41731_;
                let _e968 = bitcast<vec3<u32>>(_e967);
                let _e982 = bitcast<vec3<i32>>(vec3<u32>(((_e968.x >> bitcast<u32>(0)) & 255u), ((_e968.y >> bitcast<u32>(0)) & 255u), ((_e968.z >> bitcast<u32>(0)) & 255u)));
                let _e987 = (((_e982 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(7, 7, 7)));
                let _e988 = (_e982 == vec3<i32>(0, 0, 0));
                if _e988.x {
                    phi_41732_ = 0;
                } else {
                    phi_41732_ = _e987.x;
                }
                let _e992 = phi_41732_;
                if _e988.y {
                    phi_41733_ = 0;
                } else {
                    phi_41733_ = _e987.y;
                }
                let _e996 = phi_41733_;
                if _e988.z {
                    phi_41734_ = 0;
                } else {
                    phi_41734_ = _e987.z;
                }
                let _e1000 = phi_41734_;
                let _e1001 = (_e982 == vec3<i32>(255, 255, 255));
                let _e1009 = bitcast<vec3<u32>>(_e965);
                let _e1023 = bitcast<vec3<i32>>(vec3<u32>(((_e1009.x >> bitcast<u32>(0)) & 255u), ((_e1009.y >> bitcast<u32>(0)) & 255u), ((_e1009.z >> bitcast<u32>(0)) & 255u)));
                let _e1028 = (((_e1023 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(7, 7, 7)));
                let _e1029 = (_e1023 == vec3<i32>(0, 0, 0));
                if _e1029.x {
                    phi_41750_ = 0;
                } else {
                    phi_41750_ = _e1028.x;
                }
                let _e1033 = phi_41750_;
                if _e1029.y {
                    phi_41751_ = 0;
                } else {
                    phi_41751_ = _e1028.y;
                }
                let _e1037 = phi_41751_;
                if _e1029.z {
                    phi_41752_ = 0;
                } else {
                    phi_41752_ = _e1028.z;
                }
                let _e1041 = phi_41752_;
                let _e1042 = (_e1023 == vec3<i32>(255, 255, 255));
                let _e1055 = max(((81 + (_e185 * 3)) - select(0, 1, (_e185 > _e203))), 82);
                let _e1059 = select(3, 2, ((_e185 == 0) || (_e185 == _e203)));
                local_22 = _e186;
                if (_e1059 <= 0) {
                    phi_41760_ = 0;
                } else {
                    let _e1066 = (_e1055 >> bitcast<u32>(5));
                    if ((((_e1055 + _e1059) - 1) >> bitcast<u32>(5)) == _e1066) {
                        let _e1097 = local_22[_e1066];
                        phi_41761_ = bitcast<i32>(((_e1097 >> bitcast<u32>((_e1055 & 31))) & bitcast<u32>(((1 << bitcast<u32>(_e1059)) - 1))));
                    } else {
                        let _e1068 = (_e1055 & 31);
                        let _e1069 = (32 - _e1068);
                        let _e1071 = local_22[_e1066];
                        let _e1083 = local_22[(_e1066 + 1)];
                        phi_41761_ = (bitcast<i32>(((_e1071 >> bitcast<u32>(_e1068)) & bitcast<u32>(((1 << bitcast<u32>(_e1069)) - 1)))) | (bitcast<i32>(((_e1083 >> bitcast<u32>(0)) & bitcast<u32>(((1 << bitcast<u32>((_e1059 - _e1069))) - 1)))) << bitcast<u32>(_e1069)));
                    }
                    let _e1107 = phi_41761_;
                    phi_41760_ = _e1107;
                }
                let _e1109 = phi_41760_;
                local_23 = array<i32,8u>(0, 9, 18, 27, 37, 46, 55, 64);
                let _e1111 = local_23[_e1109];
                phi_42794_ = vec3<i32>(select(_e992, 65535, _e1001.x), select(_e996, 65535, _e1001.y), select(_e1000, 65535, _e1001.z));
                phi_42791_ = vec3<i32>(select(_e1033, 65535, _e1042.x), select(_e1037, 65535, _e1042.y), select(_e1041, 65535, _e1042.z));
                phi_42788_ = _e1111;
                break;
            }
            case 26: {
                let _e506 = vec3<i32>(bitcast<i32>(((_e186.x >> bitcast<u32>(5)) & 255u)), bitcast<i32>(((_e186.x >> bitcast<u32>(15)) & 255u)), (bitcast<i32>(((_e186.x >> bitcast<u32>(25)) & 127u)) | (bitcast<i32>(((_e186.y >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(7))));
                if (_e201 != 0) {
                    let _e532 = bitcast<i32>(_e186.z);
                    let _e544 = bitcast<i32>(_e186.x);
                    let _e594 = bitcast<i32>(_e186.y);
                    phi_41623_ = (vec3<i32>(((((_e532 >> bitcast<u32>(7)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), (bitcast<i32>(((_e186.y >> bitcast<u32>(19)) & 15u)) | (((((_e594 >> bitcast<u32>(8)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4))), (((((bitcast<i32>(((_e186.y >> bitcast<u32>(18)) & 1u)) | (bitcast<i32>(((_e186.x >> bitcast<u32>(13)) & 1u)) << bitcast<u32>(1))) | (bitcast<i32>(((_e186.z >> bitcast<u32>(6)) & 1u)) << bitcast<u32>(2))) | (bitcast<i32>(((_e186.z >> bitcast<u32>(12)) & 1u)) << bitcast<u32>(3))) | (bitcast<i32>(((_e186.y >> bitcast<u32>(2)) & 1u)) << bitcast<u32>(4))) | (((((_e594 >> bitcast<u32>(1)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(5)))) + _e506);
                    phi_41611_ = (_e506 + vec3<i32>(((((_e532 >> bitcast<u32>(1)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), (bitcast<i32>(((_e186.y >> bitcast<u32>(9)) & 15u)) | (((((_e544 >> bitcast<u32>(24)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(4))), (((bitcast<i32>(((_e186.y >> bitcast<u32>(29)) & 7u)) | (bitcast<i32>(((_e186.z >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(3))) | (bitcast<i32>(((_e186.x >> bitcast<u32>(14)) & 1u)) << bitcast<u32>(4))) | (((((_e544 >> bitcast<u32>(23)) & 1) << bitcast<u32>(31u)) >> bitcast<u32>(31u)) << bitcast<u32>(5)))));
                } else {
                    let _e508 = bitcast<i32>(_e186.y);
                    phi_41623_ = (vec3<i32>(((((_e508 >> bitcast<u32>(3)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), ((((_e508 >> bitcast<u32>(13)) & 31) << bitcast<u32>(27u)) >> bitcast<u32>(27u)), ((((_e508 >> bitcast<u32>(23)) & 63) << bitcast<u32>(26u)) >> bitcast<u32>(26u))) + _e506);
                    phi_41611_ = _e506;
                }
                let _e652 = phi_41623_;
                let _e654 = phi_41611_;
                let _e655 = bitcast<vec3<u32>>(_e654);
                let _e669 = bitcast<vec3<i32>>(vec3<u32>(((_e655.x >> bitcast<u32>(0)) & 255u), ((_e655.y >> bitcast<u32>(0)) & 255u), ((_e655.z >> bitcast<u32>(0)) & 255u)));
                let _e674 = (((_e669 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(7, 7, 7)));
                let _e675 = (_e669 == vec3<i32>(0, 0, 0));
                if _e675.x {
                    phi_41612_ = 0;
                } else {
                    phi_41612_ = _e674.x;
                }
                let _e679 = phi_41612_;
                if _e675.y {
                    phi_41613_ = 0;
                } else {
                    phi_41613_ = _e674.y;
                }
                let _e683 = phi_41613_;
                if _e675.z {
                    phi_41614_ = 0;
                } else {
                    phi_41614_ = _e674.z;
                }
                let _e687 = phi_41614_;
                let _e688 = (_e669 == vec3<i32>(255, 255, 255));
                let _e696 = bitcast<vec3<u32>>(_e652);
                let _e710 = bitcast<vec3<i32>>(vec3<u32>(((_e696.x >> bitcast<u32>(0)) & 255u), ((_e696.y >> bitcast<u32>(0)) & 255u), ((_e696.z >> bitcast<u32>(0)) & 255u)));
                let _e715 = (((_e710 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(7, 7, 7)));
                let _e716 = (_e710 == vec3<i32>(0, 0, 0));
                if _e716.x {
                    phi_41630_ = 0;
                } else {
                    phi_41630_ = _e715.x;
                }
                let _e720 = phi_41630_;
                if _e716.y {
                    phi_41631_ = 0;
                } else {
                    phi_41631_ = _e715.y;
                }
                let _e724 = phi_41631_;
                if _e716.z {
                    phi_41632_ = 0;
                } else {
                    phi_41632_ = _e715.z;
                }
                let _e728 = phi_41632_;
                let _e729 = (_e710 == vec3<i32>(255, 255, 255));
                let _e742 = max(((81 + (_e185 * 3)) - select(0, 1, (_e185 > _e203))), 82);
                let _e746 = select(3, 2, ((_e185 == 0) || (_e185 == _e203)));
                local_24 = _e186;
                if (_e746 <= 0) {
                    phi_41640_ = 0;
                } else {
                    let _e753 = (_e742 >> bitcast<u32>(5));
                    if ((((_e742 + _e746) - 1) >> bitcast<u32>(5)) == _e753) {
                        let _e784 = local_24[_e753];
                        phi_41641_ = bitcast<i32>(((_e784 >> bitcast<u32>((_e742 & 31))) & bitcast<u32>(((1 << bitcast<u32>(_e746)) - 1))));
                    } else {
                        let _e755 = (_e742 & 31);
                        let _e756 = (32 - _e755);
                        let _e758 = local_24[_e753];
                        let _e770 = local_24[(_e753 + 1)];
                        phi_41641_ = (bitcast<i32>(((_e758 >> bitcast<u32>(_e755)) & bitcast<u32>(((1 << bitcast<u32>(_e756)) - 1)))) | (bitcast<i32>(((_e770 >> bitcast<u32>(0)) & bitcast<u32>(((1 << bitcast<u32>((_e746 - _e756))) - 1)))) << bitcast<u32>(_e756)));
                    }
                    let _e794 = phi_41641_;
                    phi_41640_ = _e794;
                }
                let _e796 = phi_41640_;
                local_25 = array<i32,8u>(0, 9, 18, 27, 37, 46, 55, 64);
                let _e798 = local_25[_e796];
                phi_42794_ = vec3<i32>(select(_e679, 65535, _e688.x), select(_e683, 65535, _e688.y), select(_e687, 65535, _e688.z));
                phi_42791_ = vec3<i32>(select(_e720, 65535, _e729.x), select(_e724, 65535, _e729.y), select(_e728, 65535, _e729.z));
                phi_42788_ = _e798;
                break;
            }
            case 30: {
                if (_e201 != 0) {
                    phi_41503_ = vec3<i32>(bitcast<i32>(((_e186.z >> bitcast<u32>(7)) & 63u)), ((bitcast<i32>(((_e186.y >> bitcast<u32>(19)) & 15u)) | (bitcast<i32>(((_e186.x >> bitcast<u32>(11)) & 1u)) << bitcast<u32>(4))) | (bitcast<i32>(((_e186.x >> bitcast<u32>(31)) & 1u)) << bitcast<u32>(5))), ((((bitcast<i32>(((_e186.x >> bitcast<u32>(12)) & 3u)) | (bitcast<i32>(((_e186.x >> bitcast<u32>(23)) & 1u)) << bitcast<u32>(2))) | (bitcast<i32>(((_e186.y >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(3))) | (bitcast<i32>(((_e186.y >> bitcast<u32>(2)) & 1u)) << bitcast<u32>(4))) | (bitcast<i32>(((_e186.y >> bitcast<u32>(1)) & 1u)) << bitcast<u32>(5))));
                    phi_41491_ = vec3<i32>(bitcast<i32>(((_e186.z >> bitcast<u32>(1)) & 63u)), ((bitcast<i32>(((_e186.y >> bitcast<u32>(9)) & 15u)) | (bitcast<i32>(((_e186.x >> bitcast<u32>(24)) & 1u)) << bitcast<u32>(4))) | (bitcast<i32>(((_e186.x >> bitcast<u32>(21)) & 1u)) << bitcast<u32>(5))), (((bitcast<i32>(((_e186.y >> bitcast<u32>(29)) & 7u)) | (bitcast<i32>(((_e186.z >> bitcast<u32>(0)) & 1u)) << bitcast<u32>(3))) | (bitcast<i32>(((_e186.x >> bitcast<u32>(14)) & 1u)) << bitcast<u32>(4))) | (bitcast<i32>(((_e186.x >> bitcast<u32>(22)) & 1u)) << bitcast<u32>(5))));
                } else {
                    phi_41503_ = vec3<i32>(bitcast<i32>(((_e186.y >> bitcast<u32>(3)) & 63u)), bitcast<i32>(((_e186.y >> bitcast<u32>(13)) & 63u)), bitcast<i32>(((_e186.y >> bitcast<u32>(23)) & 63u)));
                    phi_41491_ = vec3<i32>(bitcast<i32>(((_e186.x >> bitcast<u32>(5)) & 63u)), bitcast<i32>(((_e186.x >> bitcast<u32>(15)) & 63u)), bitcast<i32>(((_e186.x >> bitcast<u32>(25)) & 63u)));
                }
                let _e339 = phi_41503_;
                let _e341 = phi_41491_;
                let _e342 = bitcast<vec3<u32>>(_e341);
                let _e356 = bitcast<vec3<i32>>(vec3<u32>(((_e342.x >> bitcast<u32>(0)) & 63u), ((_e342.y >> bitcast<u32>(0)) & 63u), ((_e342.z >> bitcast<u32>(0)) & 63u)));
                let _e361 = (((_e356 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(5, 5, 5)));
                let _e362 = (_e356 == vec3<i32>(0, 0, 0));
                if _e362.x {
                    phi_41492_ = 0;
                } else {
                    phi_41492_ = _e361.x;
                }
                let _e366 = phi_41492_;
                if _e362.y {
                    phi_41493_ = 0;
                } else {
                    phi_41493_ = _e361.y;
                }
                let _e370 = phi_41493_;
                if _e362.z {
                    phi_41494_ = 0;
                } else {
                    phi_41494_ = _e361.z;
                }
                let _e374 = phi_41494_;
                let _e375 = (_e356 == vec3<i32>(63, 63, 63));
                let _e383 = bitcast<vec3<u32>>(_e339);
                let _e397 = bitcast<vec3<i32>>(vec3<u32>(((_e383.x >> bitcast<u32>(0)) & 63u), ((_e383.y >> bitcast<u32>(0)) & 63u), ((_e383.z >> bitcast<u32>(0)) & 63u)));
                let _e402 = (((_e397 << bitcast<vec3<u32>>(vec3<i32>(15, 15, 15))) + vec3<i32>(16384, 16384, 16384)) >> bitcast<vec3<u32>>(vec3<i32>(5, 5, 5)));
                let _e403 = (_e397 == vec3<i32>(0, 0, 0));
                if _e403.x {
                    phi_41510_ = 0;
                } else {
                    phi_41510_ = _e402.x;
                }
                let _e407 = phi_41510_;
                if _e403.y {
                    phi_41511_ = 0;
                } else {
                    phi_41511_ = _e402.y;
                }
                let _e411 = phi_41511_;
                if _e403.z {
                    phi_41512_ = 0;
                } else {
                    phi_41512_ = _e402.z;
                }
                let _e415 = phi_41512_;
                let _e416 = (_e397 == vec3<i32>(63, 63, 63));
                let _e429 = max(((81 + (_e185 * 3)) - select(0, 1, (_e185 > _e203))), 82);
                let _e433 = select(3, 2, ((_e185 == 0) || (_e185 == _e203)));
                local_26 = _e186;
                if (_e433 <= 0) {
                    phi_41520_ = 0;
                } else {
                    let _e440 = (_e429 >> bitcast<u32>(5));
                    if ((((_e429 + _e433) - 1) >> bitcast<u32>(5)) == _e440) {
                        let _e471 = local_26[_e440];
                        phi_41521_ = bitcast<i32>(((_e471 >> bitcast<u32>((_e429 & 31))) & bitcast<u32>(((1 << bitcast<u32>(_e433)) - 1))));
                    } else {
                        let _e442 = (_e429 & 31);
                        let _e443 = (32 - _e442);
                        let _e445 = local_26[_e440];
                        let _e457 = local_26[(_e440 + 1)];
                        phi_41521_ = (bitcast<i32>(((_e445 >> bitcast<u32>(_e442)) & bitcast<u32>(((1 << bitcast<u32>(_e443)) - 1)))) | (bitcast<i32>(((_e457 >> bitcast<u32>(0)) & bitcast<u32>(((1 << bitcast<u32>((_e433 - _e443))) - 1)))) << bitcast<u32>(_e443)));
                    }
                    let _e481 = phi_41521_;
                    phi_41520_ = _e481;
                }
                let _e483 = phi_41520_;
                local_27 = array<i32,8u>(0, 9, 18, 27, 37, 46, 55, 64);
                let _e485 = local_27[_e483];
                phi_42794_ = vec3<i32>(select(_e366, 65535, _e375.x), select(_e370, 65535, _e375.y), select(_e374, 65535, _e375.z));
                phi_42791_ = vec3<i32>(select(_e407, 65535, _e416.x), select(_e411, 65535, _e416.y), select(_e415, 65535, _e416.z));
                phi_42788_ = _e485;
                break;
            }
            default: {
                phi_42794_ = vec3<i32>(0, 0, 0);
                phi_42791_ = vec3<i32>(0, 0, 0);
                phi_42788_ = 0;
                break;
            }
        }
        let _e3581 = phi_42794_;
        let _e3583 = phi_42791_;
        let _e3585 = phi_42788_;
        phi_42793_ = _e3581;
        phi_42790_ = _e3583;
        phi_42787_ = _e3585;
    }
    let _e4201 = phi_42793_;
    let _e4203 = phi_42790_;
    let _e4205 = phi_42787_;
    let _e4217 = ((((((vec3<i32>((64 - _e4205)) * _e4201) + (vec3<i32>(_e4205) * _e4203)) + vec3<i32>(32, 32, 32)) >> bitcast<vec3<u32>>(vec3<i32>(6, 6, 6))) * vec3<i32>(31, 31, 31)) >> bitcast<vec3<u32>>(vec3<i32>(6, 6, 6)));
    uOutput = vec4<f32>(unpack2x16float(bitcast<u32>(_e4217.x)).x, unpack2x16float(bitcast<u32>(_e4217.y)).x, unpack2x16float(bitcast<u32>(_e4217.z)).x, unpack2x16float(15360u).x);
    return;
}

@fragment 
fn main(@builtin(position) gl_FragCoord: vec4<f32>, @location(0) param: vec2<f32>) -> @location(0) vec4<f32> {
    gl_FragCoord_1 = gl_FragCoord;
    global = param;
    main_1();
    let _e5 = uOutput;
    return _e5;
}
