using System;
using System.Collections.Generic;
using LobCorp.Save.Type;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Parsers.Json
{
	public class UnlimitedJsonSaveParser : JsonSaveParserBase
	{
		public UnlimitedJsonSaveParser() : base(SaveType.Unlimited)
		{
		}
		protected override Dictionary<string, object> Parse(JObject save)
		{
			throw new NotImplementedException();
		}
	}
}
