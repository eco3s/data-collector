macro_rules! tree_gen {
	(
		@ $output:path[$item_output:path]
        ($prefix:ident) $tree:ident {
            #[$meta:meta]
            $($field:tt |> $level:ident)+
        }
	) => {
		paste::paste! {
            $(
                #[derive(serde::Deserialize, Debug)]
                pub struct [<
                    $prefix:camel $level:camel
                >] {
                    #[serde(rename(
                        deserialize = "r" $field "_knm"
                    ))]
                    #[$meta]
                    ko: Option<String>,

                    #[serde(rename(
                        deserialize = "r" $field "_nm"
                    ))]
                    en: Option<String>,
                }

                impl From<
                    [<$prefix:camel $level:camel>]
                > for $item_output {
                    fn from([<
                        $prefix:camel
                        $level:camel
                    >] {
                        en, ko
                    }: [<
                        $prefix:camel
                        $level:camel
                    >]) -> Self { Self { en, ko } }
                }
            )+

			#[derive(serde::Deserialize, Debug)]
			pub struct [<
                $prefix:camel
                $tree:camel
            >] {
                $(
                    #[serde(flatten)]
                    [<$level:snake>]: [<
                        $prefix:camel
                        $level:camel
                    >],
                )+
			}

			impl From<[<
                $prefix:camel $tree:camel
            >]> for $output {
				fn from(value: [<
                    $prefix:camel $tree:camel
                >]) -> Self {
					Self {
                        $(
                            [<$level:snake>]: value
                                .[<$level:snake>]
                                .into(),
                        )+
					}
				}
			}
		}
	};
}

pub(crate) use tree_gen;

macro_rules! docs_gen {
	($name:ident[$($num:literal)->+]) => {
		paste::paste! {
			#[derive(Debug)]
			struct [<$name:camel>](Vec<String>);

			impl<'de> serde::de::Deserialize<'de> for [<
                $name:camel
            >] {
				fn deserialize<D>(
                    deserializer: D
                ) -> Result<Self, D::Error>
				where D: serde::de::Deserializer<'de> {
					#[derive(serde::Deserialize)]
					#[serde(
                        field_identifier,
                        rename_all = "lowercase"
                    )]
					enum Field {
                        $(
                            #[serde(rename(
                                deserialize = "ref_doc" $num
                            ))]
                            [<$name:camel $num>],
                        )+

						#[serde(other)]
						Ignore,
					}

					struct Visitor;

					impl<'de> serde::de::Visitor<'de> for Visitor {
						type Value = [<$name:camel>];

						fn expecting(
							&self,
							formatter: &mut std::fmt::Formatter,
						) -> std::fmt::Result {
							formatter.write_str(
                                stringify!(struct [<$name:camel>])
                            )
						}

						fn visit_map<V>(
							self,
							mut map: V,
						) -> Result<Self::Value, V::Error>
						where
							V: serde::de::MapAccess<'de>,
						{
							Ok([<$name:camel>](
								std::iter::from_fn(|| {
									match map.next_key::<Field>() {
										Ok(v) => v.map(|_| map.next_value()),
										Err(e) => Some(Err(e)),
									}
								})
								.filter(|s| match s {
									Ok(s) => !String::is_empty(&s),
									_ => false,
								})
								.collect::<Result<Vec<_>, V::Error>>()?,
							))
						}
					}

					const FIELDS: &'static [&'static str] = &[
                        $(
                            stringify!([<ref_doc $num>]),
                        )+
                    ];
					deserializer.deserialize_struct(
                        stringify!([<name:camel>]), FIELDS, Visitor
                    )
				}
			}
		}
	};
}

pub(crate) use docs_gen;

#[allow(unused)]
macro_rules! swap {
	($x:expr, $y:expr) => {{
		let tmp = $y;

		($x, tmp)
	}};
}

#[allow(unused)] pub(crate) use swap;
